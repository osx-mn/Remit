import DynamicTitle from "../Atoms/DynamicTitle";

interface EditUserNameMoleculeProps {
    userName: string;
    onClick?: () => void;
}

const EditUserNameMolecule: React.FC<EditUserNameMoleculeProps> = ({ userName, onClick }) =>{
    return(
        /* móvil: ancho completo | PC: 60% centrado */
        <div className="flex items-center justify-between bg-[#252525] w-full md:w-[60%] min-w-0 px-3 py-[5px] mb-[5px] rounded-[5px] gap-2">
            <DynamicTitle title={userName}/>
            <button onClick={onClick} className="shrink-0 border border-white rounded-[3px] h-[30px] px-[8px] text-white text-sm">EDIT NAME</button>
        </div>
    )
}

export default EditUserNameMolecule;