interface InputTypeProps {
    type: string;
    placeholder?: string;
    value: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

const InputType: React.FC<InputTypeProps> = ({type, placeholder= "", value, onChange}) => {
    return(
        <input
        className="w-full h-full border border-white rounded-[5px] text-white text-[12px] p-[5px]"
        type={type}
        placeholder={placeholder}
        value={value}
        onChange={onChange} />
    )
}

export default InputType;