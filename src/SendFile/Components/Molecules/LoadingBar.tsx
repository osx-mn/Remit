import { forwardRef, useImperativeHandle, useRef } from "react";

export interface LoadingBarHandle {
    setProgress: (value: number) => void;
    showBar: (value: boolean) => void;
}

const LoadingBar = forwardRef<LoadingBarHandle>(({}, ref) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const barRef = useRef<HTMLDivElement>(null);
    const textRef = useRef<HTMLParagraphElement>(null);

    useImperativeHandle(ref, () => ({

        showBar: (value: boolean) => {
            if (containerRef.current){
                containerRef.current.style.opacity = value? "1" : "0";
            }
        },

        setProgress: (value: number) => {
            if (barRef.current) {
                barRef.current.style.width = `${value.toFixed(0)}%`;
            }
            if (textRef.current) {
                textRef.current.innerText = `${value.toFixed(0)}%`;
            }
        }
    }));

    return (
        <div className="flex w-[60%] h-[20px] mt-[25px] success-transfer" style={{ opacity: 0}} ref={containerRef}>
            <div className="flex items-center w-full h-full rounded-[5px] bg-[#333E48]">
                <div 
                    ref={barRef}
                    className="h-[10px] mx-[10px] rounded-[5px] bg-[#51C1A4] transition-all duration-100 ease-linear"
                    style={{ width: '0%' }}
                ></div>
            </div>
            <p 
                ref={textRef}
                className="text-white text-[12px] px-[5px] rounded-[5px] ml-[10px] bg-[#333E48]"
            >
                0%
            </p>
        </div>
    )
});

export default LoadingBar;