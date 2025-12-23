import LoadFiles from "../Molecules/LoadFiles";
import EditUserName from "../Molecules/EditUserName";

interface FilesContainerProps {
    onClick: () => void;
    username: string;
}

const FilesContainer: React.FC<FilesContainerProps> = ({onClick, username}) => {
    return (
        <div className="flex grow flex-col h-full justify-center items-center">
            <EditUserName onClick={onClick} userName={username} />
            <LoadFiles />
            <button className="px-5 py-1 mt-5 border border-white rounded-sm">
                <p className="text-white">Enviar archivos</p>
            </button>
        </div>
    )
}

export default FilesContainer;