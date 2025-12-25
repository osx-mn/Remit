import DevicesCard from "../Molecules/DevicesCard";

interface Dispositivo {
    full_name: string;
    disp_name: string;
    ip: string;
    port: number;
    properties: Array<[String, String]>;
}

interface DevicesContainerProps {
    devicesList: Dispositivo[];
}

const DevicesContainer: React.FC<DevicesContainerProps> = ({devicesList}) =>{

    const getDeviceIp = (ip: string) => {
        console.log("ip presionado: ", ip);
    }

    return(
        <div className="flex flex-col items-center w-[250px] h-[98%] bg-[#252525] rounded-[5px] ml-[5px]">
            <p className="text-white text-center text-[24px]">Dispositivos</p>
            {devicesList.map((device, _) => {
                return <DevicesCard key={device.full_name} deviceProps={device} getDeviceIp={getDeviceIp}/>
            })}
        </div>
    )
}

export default DevicesContainer;