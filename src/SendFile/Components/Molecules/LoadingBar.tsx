interface LoadingBarProps {
    porcentaje: number;
    showState: boolean;
}

const LoadingBar: React.FC<LoadingBarProps> = ({porcentaje, showState}) =>{

    const cooldown = Number(porcentaje.toFixed(0)) % 10 === 0 ? porcentaje : 0;

    return (
        <div className={`flex w-[60%] h-[20px] mt-[25px] success-transfer ${showState? "show" : ""}`}>
            <div className="flex items-center w-full h-full rounded-[5px] bg-[#333E48]">
                <div className={`w-[${cooldown.toFixed(0)}%] h-[10px] mx-[10px] rounded-[5px] bg-[#51C1A4] `}></div>
            </div>
            <p className="text-white text-[12px] px-[5px] rounded-[5px] ml-[10px] bg-[#333E48]">{porcentaje.toFixed(0)}%</p>
        </div>
    )
}

export default LoadingBar;