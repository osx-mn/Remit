use crate::MdnsState;

use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo, TxtProperties};
use serde::Serialize;
use tauri::{command, Emitter};

#[derive(Debug, Serialize, Clone)]
pub struct Dispositivo {
    pub disp_name: String,
    pub ip: String,
    pub port: u16,
    pub properties: Vec<(String, String)>,
}

#[command]
pub fn find_devices(app_handle: tauri::AppHandle, state: tauri::State<MdnsState>) {
    let daemon_state = state.daemon.clone();

    tauri::async_runtime::spawn(async move {
        println!("Iniciando daemon...");
        let ty_domain: &str = "_remit_transfer._tcp.local.";
        let instance_name: &str = "Remit";
        let hostname: &str = "Remit.local.";
        let ip: String = local_ip().unwrap().to_string();

        let port = 8989;
        let properties = [("version", "1.0")];

        let service_info = ServiceInfo::new(
            ty_domain,
            instance_name,
            hostname,
            ip,
            port,
            &properties[..],
        )
        .unwrap();

        let mdns_daemon = ServiceDaemon::new().unwrap();
        //guardar daemon en state para shutdown global
        *daemon_state.lock().unwrap() = Some(mdns_daemon.clone());

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
                    let ip = resolved
                        .get_addresses()
                        .iter()
                        .next()
                        .map(|ip| ip.to_string())
                        .unwrap_or_default();

                    let port = resolved.get_port();

                    let properties: Vec<_> = resolved
                        .txt_properties
                        .clone()
                        .into_property_map_str()
                        .into_iter()
                        .collect();

                    let dispositivo = Dispositivo {
                        disp_name: resolved.get_hostname().to_string(),
                        ip: ip.clone(),
                        port: resolved.get_port(),
                        properties: properties.clone(),
                    };

                    println!("Dispositivo encontrado: {}", resolved.get_hostname());
                    println!("IP: {}", ip.clone());
                    println!("Port: {}", resolved.get_port());
                    println!("Properties: {:#?}", properties.clone());

                    println!("Dispositivo encontrado: {}", dispositivo.disp_name);
                    let _ = app_handle.emit("mdns-device-found", dispositivo);
                }

                ServiceEvent::ServiceRemoved(removed, otra_info) => {
                    println!("Dispositivo removido: {}", removed);
                    println!("Otra info: {}", otra_info);

                    // let _ = app_handle.emit("mdns-device-removed", "");
                }
                _ => {}
            }
        }
    });
}
