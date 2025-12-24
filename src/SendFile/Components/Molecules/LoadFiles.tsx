import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

interface LoadFilesProps{
    onFileSelect: (filePath: string) => void;
}

const LoadFiles: React.FC<LoadFilesProps> = ({onFileSelect}) => {
    const [fileName, setFileName] = useState<string>("Ningún archivo seleccionado");

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
        <div className="w-3/5 h-2/5 bg-stone-950 rounded-md ml-1 flex items-center justify-center">
            <div className="border border-stone-600 w-[95%] h-[95%] rounded-md flex flex-col items-center justify-center">
                <p className="text-white text-center text-2xl">Cargar archivos</p>

                <button className="px-5 py-1 mt-5 border border-white rounded-sm text-white" onClick={handleFileSelect}>Seleccionar archivos</button>
                <p className="text-white mt-5">{fileName.split(/[\\/]/).pop() ?? ""}</p>
            </div>
        </div>
    );
};

export default LoadFiles;