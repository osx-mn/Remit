import { createContext, useContext, useState } from "react";

interface deviceContextType{
    deviceSelected: boolean;
    setDeviceSelected: (value: boolean) => void;
    deviceSelectedIp: string;
    setDeviceSelectedIp: (value: string) => void;
}

const DeviceContext = createContext<deviceContextType | null>(null);

export const DeviceProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [deviceSelected, setDeviceSelected] = useState(false);
    const [deviceSelectedIp, setDeviceSelectedIp] = useState("");

    return (
        <DeviceContext.Provider value={{ deviceSelected, setDeviceSelected, deviceSelectedIp, setDeviceSelectedIp }}>
            {children}
        </DeviceContext.Provider>
    )
}

export const useDevice = () => {
    const context= useContext(DeviceContext);
    if(!context){
        throw new Error("useDevice debe usarse dentro de DeviceProvider");
    }

    return context;
}