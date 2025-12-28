import {useState} from 'react';
import { invoke } from "@tauri-apps/api/core";

import InputWithLabel from "../Molecules/InputWithLabel";
import BtnAutosize from "../Atoms/BtnAutosize";

interface ModalProps {
    ModalActive: boolean;
    onClick: () => void;
    onNameChange: () => void;
}

const Modal: React.FC<ModalProps> = ({ModalActive, onClick, onNameChange}) => {

    const [inputName, setInputName] = useState<string>("");

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) =>{
        setInputName(e.target.value);
    }

    const setUserName = async () =>{
        try{
            await invoke<string>("change_username", {newName: inputName});
        } catch (error){
            console.error("Error al establecer el nombre desde rust:", error);
        }
    }

    const handleSubmit = async () =>{
        if (inputName.trim().length != 0){
            setUserName();
            setInputName("");
            onClick();
            await onNameChange();
        }
    }

    const handleCancel = () => {
        setInputName("");
        onClick(); // cierra el modal
    };

    return(
        <div id="modal" className={`${ModalActive ? 'flex flex-col' : 'hidden'} fixed w-[400px] h-[120px] ml-[40%] z-10 items-center justify-center bg-[#303030] rounded-[5px]`}>
            <InputWithLabel
            id="input_name"
            label="NOMBRE DE USUARIO"
            type="text"
            placeholder="Nombre de usuario"
            value={inputName}
            onChange={(e) => handleChange(e)}
            />
            
            <div className='w-full flex items-center justify-center'>
                <BtnAutosize
                id="btn_send"
                x="100"
                y="50"
                text="Cambiar nombre"
                onclick={handleSubmit}/>

                <BtnAutosize
                id="btn_cancel"
                x="100"
                y="50"
                text="Cancelar"
                onclick={handleCancel}/>
            </div>
        </div>
    )
}

export default Modal;