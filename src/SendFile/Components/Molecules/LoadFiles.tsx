import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import { useDevice } from "../../../context/DeviceContext";

interface LoadFilesProps{
    onFileSelect: (filePath: string) => void;
}

const LoadFiles: React.FC<LoadFilesProps> = ({ onFileSelect }) => {
    const [fileName, setFileName] = useState<string>("Ningún archivo seleccionado");

    //leer el contetxo global de dispositivo seleccionado
    const { deviceSelected }= useDevice();

    //función para seleccionar archivo con ruta absoluta del plugin tauri-plugin-dialog 
    const handleFileSelect = async () => {
        try{
            const selectFilePath = await open({
                multiple: false,
                directory: false,
            })
            if(selectFilePath){
                setFileName(selectFilePath);
                onFileSelect(selectFilePath);
            }
        }catch(error){
            console.error("Error al seleccionar el archivo:", error);
        }
    };
    
    return (
        /* móvil: ancho completo con altura fija cómoda | PC: 60% de ancho con altura proporcional */
        <div className="w-full md:w-3/5 h-[200px] md:h-2/5 bg-stone-950 rounded-md flex items-center justify-center mt-3">
            <div className="border border-stone-600 w-[95%] h-[95%] rounded-md flex flex-col items-center justify-center">
                <p className="text-white text-center text-2xl">Cargar archivos</p>

                <button
                    className="px-5 py-1 mt-5 border border-white rounded-sm text-white disabled:opacity-30 disabled:border-gray-500 disabled:cursor-not-allowed"
                    onClick={handleFileSelect}
                    disabled={!deviceSelected}>
                        Seleccionar archivos</button>

                <p className="text-white text-center mt-5 px-2 max-w-full break-all">{fileName.split(/[\\\/]/).pop() ?? ""}</p>
            </div>
        </div>
    );
};

export default LoadFiles;