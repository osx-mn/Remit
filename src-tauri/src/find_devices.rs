use std::eprint;

use crate::MdnsState;

use crate::backend_db;
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

    // Guard: si ya existe un daemon mDNS activo, no crear otro
    {
        let guard = daemon_state.lock().unwrap();
        if guard.is_some() {
            println!("Daemon mDNS ya activo, ignorando llamada duplicada.");
            return;
        }
    }

    tauri::async_runtime::spawn(async move {
        println!("Iniciando daemon...");
        let ty_domain: &str = "_remit_transfer._tcp.local.";

        let nombre_dispositivo = match backend_db::user_app() {
            Ok(nombre) => nombre,
            Err(e) => {
                eprintln!("Error al leer el nombre: {}", e);
                String::from("Invitado")
            }
        };

        let instance_name: &str = nombre_dispositivo.as_str();

        //Definir hostname único por dispositivo
        let sys_hostname = hostname::get()
            .map(|h| h.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "remit-device".to_string());
        let hostname_mdns = format!("{}.local.", sys_hostname);
        
        let this_device_ip: String = match local_ip(){
            Ok(ip) => ip.to_string(),
            Err(e) => { eprintln!("Error IP: {}", e); return; }
        };

        let this_device_port = 8989;

        let properties = [("nombre_dispositivo", nombre_dispositivo.clone())];

        let service_info = ServiceInfo::new(
            ty_domain,
            instance_name,
            &hostname_mdns,
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
                        .find(|ip| ip.is_ipv4())
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

                    //Determinar que los dispositivos encontrados sean diferentes del actual
                    if resolved.get_fullname() != service_full_name {
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
