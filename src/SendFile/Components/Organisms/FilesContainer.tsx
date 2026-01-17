import { useEffect, useState } from "react";
import LoadFiles from "../Molecules/LoadFiles";
import EditUserName from "../Molecules/EditUserName";
import { invoke } from "@tauri-apps/api/core";

import { useDevice } from "../../../context/DeviceContext";

interface FilesContainerProps {
    onClick: () => void;
    username: string;
}

const FilesContainer: React.FC<FilesContainerProps> = ({onClick, username}) => {

    const [selectedFile, setSelectedFile] = useState<string>("");
    const { deviceSelected, deviceSelectedIp }= useDevice();
    const [sendState, setSendState] = useState<string>("");

    useEffect(() => {
        invoke("ftp_server");
    }, []);

    const handleSendFile = async (fileName: string) => {
        console.log("ip destino: ",deviceSelectedIp)
        await invoke("ftp_client", {
            filePath: fileName,
            targetDevice: deviceSelectedIp,
        })
        setSendState("show");
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
            <p className={`bg-[#51C1A4] px-[25px] py-[5px] rounded-[5px] mt-[20px] text-[#333E48] success-send-file ${sendState}`}>Archivo enviado</p>
        </div>
    )
}

export default FilesContainer;