import { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const LoadFiles: React.FC = () => {
    const [fileName, setFileName] = useState<string>("Ningún archivo seleccionado");

    const handleFileSelect = async () => {
        try{
            const selectFilePath = await open({
                multiple: false,
                directory: false,
            })
            if(selectFilePath){
                setFileName(selectFilePath);
                invoke("get_in_backend", {
                    filePath: selectFilePath,
                })
            }
        }catch(error){
            console.error("Error al seleccionar el archivo:", error);
        }
    };
    
    return (
        <div className="w-3/5 h-2/5 bg-stone-950 rounded-md ml-1 flex items-center justify-center">
            <div className="border border-stone-600 w-[95%] h-[95%] rounded-md flex flex-col items-center justify-center">
                <p className="text-white text-center text-2xl">Cargar archivos</p>

                <button onClick={handleFileSelect}>Seleccionar archivos</button>
                <p className="text-white mt-5">{fileName}</p>
            </div>
        </div>
    );
};

export default LoadFiles;