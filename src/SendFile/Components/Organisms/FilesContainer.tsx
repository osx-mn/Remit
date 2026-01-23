import { useEffect, useState, useRef } from "react";
import LoadFiles from "../Molecules/LoadFiles";
import EditUserName from "../Molecules/EditUserName";
import LoadingBar, { LoadingBarHandle } from "../Molecules/LoadingBar";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import { useDevice } from "../../../context/DeviceContext";

interface FilesContainerProps {
    onClick: () => void;
    username: string;
}

const FilesContainer: React.FC<FilesContainerProps> = ({onClick, username}) => {

    const [selectedFile, setSelectedFile] = useState<string>("");
    const { deviceSelected, deviceSelectedIp }= useDevice();
    const [transferState, setTransferState] = useState<boolean>(false);
    
    const [stateMessage, setStateMessage] = useState<string>("");

    //Cargar la referencia de la barra de carga
    const loadingBarRef = useRef<LoadingBarHandle>(null);

    useEffect(() => {
        let unlisten: (() => void) | null = null;

        const startTransfer = async () => {

            await invoke("ftp_server");

            unlisten = await listen<number>("send_percentage", (event) => {
                if (event.payload !== undefined) {
                    
                    if (loadingBarRef.current) {
                        loadingBarRef.current.showBar(true);
                        loadingBarRef.current.setProgress(event.payload);
                    }

                    if (event.payload >= 100) {
                        loadingBarRef.current?.showBar(false);
                        setStateMessage("Archivo Enviado!");
                        setTransferState(true);
                    }
                }
            });
        };

        startTransfer();

        // Cleanup
        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    }, []);


    const handleSendFile = async (fileName: string) => {
        await invoke("ftp_client", {
            filePath: fileName,
            targetDevice: deviceSelectedIp,
        })
    }

    console.log("disables state: ", deviceSelected);

    return (
        <div className="flex grow flex-col h-full justify-center items-center">
            <EditUserName onClick={onClick} userName={username} />
            <LoadFiles onFileSelect={setSelectedFile}/>
            <button 
                className="px-5 py-1 mt-5 border border-white rounded-sm disabled:opacity-30 disabled:border-gray-500 disabled:cursor-not-allowed"
                onClick={() => { handleSendFile(selectedFile)}}
                disabled={!deviceSelected}>
                <p className="text-white">Enviar archivos</p>
            </button>
            <LoadingBar ref={loadingBarRef}/>
            <p className={`px-[25px] py-[5px] rounded-[5px] mt-[20px] success-transfer ${transferState? "show-send" : ""}`} onAnimationEnd={() => setTransferState(false)}>{stateMessage}</p>
        </div>
    )
}

export default FilesContainer;