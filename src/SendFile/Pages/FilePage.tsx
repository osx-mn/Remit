    import { useState, useEffect, useRef } from "react";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    import { DeviceProvider } from "../../context/DeviceContext";

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

        // Guard: evita que find_devices se invoque más de una vez
        const findDevicesCalledRef = useRef(false);

        //Funcion para obtener el nombre del usuario desde rust
        const fetchUserName = async () => {
            try {
                const respuesta = await invoke<string>("user_app");
                setUsername(respuesta);
            } catch (error) {
                console.error("Error al obtener el nombre desde rust:", error);
            }
        }

        useEffect(() => {
            fetchUserName();

            let unlistenFound: (() => void) | null = null;
            let unlistenRemove: (() => void) | null = null;

            const setup = async () => {
                // Detecta un dispositivo en la red de tipo remit
                unlistenFound = await listen<Dispositivo>(
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

                // Detecta cuando un dispositivo se remueve de la red
                unlistenRemove = await listen<string>(
                    "mdns-device-removed",
                    (event) => {
                        setDevices(prev => {
                            const next: Map<string, Dispositivo> = new Map(prev);
                            next.delete(event.payload);
                            console.log("Dispositivo removido: ", event.payload);
                            return next;
                        });
                    }
                );

                // Invocar find_devices solo una vez aunque el componente se monte dos veces
                if (!findDevicesCalledRef.current) {
                    findDevicesCalledRef.current = true;
                    await invoke("find_devices");
                }
            };

            setup();

            // Cleanup: desregistra los listeners al desmontar el componente
            return () => {
                unlistenFound?.();
                unlistenRemove?.();
            };
        }, []);

        //ver los cambios en Dispositivos
        useEffect(() => {
            console.log("Devices: ", devices);
        }, [devices]);

        const handleModal = () =>{
            setModalActive(true);
        }

        const handleModalClose = () =>{
            setModalActive(false);
        }

        return(
            <DeviceProvider>
                {/* móvil: columna con scroll vertical | PC: fila sin scroll */}
                <div className="flex flex-col md:flex-row w-full flex-1 min-h-0 bg-[#161616] overflow-y-auto md:overflow-hidden p-2 gap-2">
                    <DevicesContainer
                        key={"devContKey"}
                        devicesList={Array.from(devices.values())}/>

                    <FilesContainer
                        onClick={handleModal}
                        username={username}/>

                    <Modal ModalActive={modalActive} onClick={handleModalClose} onNameChange={fetchUserName}/>
                </div>
            </DeviceProvider>
        )
    }

    export default FilePage;