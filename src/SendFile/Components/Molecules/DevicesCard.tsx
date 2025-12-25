
import DynamicTitle from "../Atoms/DynamicTitle";

interface Dispositivo {
    full_name: string;
    disp_name: string;
    ip: string;
    port: number;
    properties: Array<[String, String]>;
}

interface DevicesCardProps{
    deviceProps: Dispositivo;
    getDeviceIp: (ip: string) => void;
}

const DevicesCard: React.FC<DevicesCardProps> = ({ deviceProps, getDeviceIp }) => {
    return(
        <button className="w-[90%] h-[40px] bg-[#303030] rounded-[5px] mt-[10px] center-v"
        onClick={() => getDeviceIp(deviceProps.ip)}>
           <DynamicTitle title={deviceProps.properties[0][1]}/>
        </button>
    )
}

export default DevicesCard;