    import { useState, useEffect } from "react";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    import DevicesContainer from "../Components/Organisms/DevicesContainer";
    import FilesContainer from "../Components/Organisms/FilesContainer";
    import Modal from "../Components/Organisms/Modal";

    interface Dispositivo {
        full_name: string;
        disp_name: string;
        ip: string;
        port: number;
        properties: Array<[string, string]>;
    }

    const FilePage: React.FC = () =>{
        
        const [modalActive, setModalActive] = useState(false);
        const [username, setUsername] = useState("Empty");
        const [devices, setDevices] = useState<Map<string, Dispositivo>>(new Map());

        //Funcion para obtener el nombre del usuario desde rust
        const fetchUserName = async () => {
            try {
                const respuesta = await invoke<string>("user_app");
                setUsername(respuesta);
            } catch (error) {
                console.error("Error al obtener el nombre desde rust:", error);
            }
        }

        //obtener la lista de dispositivos desde rust
        const get_devices_list = async() =>{

                //detecta un dispositivo en la red de tipo remit
                const unlistenFound = listen<Dispositivo>(
                    "mdns-device-found",
                    (event) => {
                        setDevices(prev => {
                            const next = new Map(prev);
                            next.set(event.payload.full_name, event.payload);
                            console.log("Dispositivo encontrado: ", event.payload);
                            return next;
                        });
                    }
                );

                //detecta cuando un dispositivo se remueve de la red de tipo remit
                const unListenRemove = listen<Dispositivo>(
                    "mdns-device-removed",
                    (event) => {
                        setDevices(prev => {
                            const next = new Map(prev);
                            next.delete(event.payload.full_name);
                            return next;
                        })
                    }
                );

                await invoke("find_devices");

                return async () => {
                    const unlistenADD = await unlistenFound;
                    const unlistenDel = await unListenRemove;
                    unlistenADD();
                    unlistenDel();
                }
            }

        useEffect(() => {
            fetchUserName();

            //ejecutar y guardar la promesa de limpieza
            const cleanupPromise = get_devices_list();
            return () => {
                cleanupPromise.then(cleanup => cleanup());
            }
        }, []);

        //ver los cambios en Dispositivos
        useEffect(() => {
            console.log("Devices: ", devices);
        }), [devices];

        const handleModal = () =>{
            setModalActive(true);
        }

        const handleModalClose = () =>{
            setModalActive(false);
        }

        return(
            <div className="flex items-center w-full h-full bg-[#161616]">
                <DevicesContainer key={devices.size} devicesList={Array.from(devices.values())}/>
                <FilesContainer onClick={handleModal} username={username}/>
                <Modal ModalActive={modalActive} onClick={handleModalClose} onNameChange={fetchUserName}/>
            </div>
        )
    }

    export default FilePage;