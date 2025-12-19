use crate::MdnsState;

use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::Serialize;
use tauri::{command, Emitter};

#[derive(Debug, Serialize, Clone)]
pub struct Dispositivo {
    pub full_name: String,
    pub disp_name: String,
    pub ip: String,
    pub port: u16,
    pub properties: Vec<(String, String)>,
}

#[command]
pub fn find_devices(app_handle: tauri::AppHandle, state: tauri::State<MdnsState>) {
    let daemon_state = state.daemon.clone();
    let service_full_name_state = state.service_full_name.clone();

    tauri::async_runtime::spawn(async move {
        println!("Iniciando daemon...");
        let ty_domain: &str = "_remit_transfer._tcp.local.";

        let hotname_ostring = hostname::get().map_err(|e| e.to_string());
        let nombre_dispositivo: String = match hotname_ostring
            .expect("Error al obtener nombre del dispositivo")
            .into_string()
        {
            Ok(host) => host,
            Err(e) => {
                eprintln!("Error al obtener nombre del dispositivo: {:?}", e);
                String::from("Error")
            }
        };

        let instance_name: &str = nombre_dispositivo.as_str();
        let hostname: &str = "Remit.local.";
        let this_device_ip: String = local_ip().unwrap().to_string();

        let this_device_port = 8989;

        let properties = [("nombre_dispositivo", nombre_dispositivo.clone())];

        let service_info = ServiceInfo::new(
            ty_domain,
            instance_name,
            hostname,
            &this_device_ip,
            this_device_port,
            &properties[..],
        )
        .unwrap();

        let service_full_name = service_info.get_fullname().to_string();
        let mdns_daemon = ServiceDaemon::new().unwrap();

        //guardar daemon y nombre del servicio en state para shutdown global desde lib.rs
        *daemon_state.lock().unwrap() = Some(mdns_daemon.clone());
        *service_full_name_state.lock().unwrap() = Some(service_full_name.clone());

        mdns_daemon.register(service_info).unwrap();

        //Buscar dispositivos
        let receiver = match mdns_daemon.browse(ty_domain) {
            Ok(receiver) => receiver,
            Err(error) => {
                eprintln!("Browse error: {}", error);
                return;
            }
        };

        //Recibir dispositivos
        println!("Recibiendo dispositivos...");
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(resolved) => {
                    let external_device_ip = resolved
                        .get_addresses()
                        .iter()
                        .next()
                        .map(|ip| ip.to_string())
                        .unwrap_or_default();

                    let external_device_port = resolved.get_port();

                    let properties: Vec<_> = resolved
                        .txt_properties
                        .clone()
                        .into_property_map_str()
                        .into_iter()
                        .collect();

                    let dispositivo = Dispositivo {
                        full_name: resolved.get_fullname().to_string(),
                        disp_name: resolved.get_hostname().to_string(),
                        ip: external_device_ip.clone(),
                        port: external_device_port,
                        properties: properties.clone(),
                    };

                    //enviar información de dispositivos encontrados diferentes al propio
                    if this_device_ip != external_device_ip {
                        let _ = app_handle.emit("mdns-device-found", dispositivo);
                    }
                }

                ServiceEvent::ServiceRemoved(_, full_name) => {
                    let _ = app_handle.emit("mdns-device-removed", full_name);
                }
                _ => {}
            }
        }
    });
}
