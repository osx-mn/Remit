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

    useEffect(() => {
        invoke("ftp_server");
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
        </div>
    )
}

export default FilesContainer;