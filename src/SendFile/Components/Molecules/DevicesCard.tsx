import { useDevice } from "../../../context/DeviceContext";
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
    deviceCardSelected: boolean;
}

const DevicesCard: React.FC<DevicesCardProps> = ({ deviceProps, getDeviceIp, deviceCardSelected}) => {

    const { deviceSelectedIp } = useDevice();
    const isSelected = deviceSelectedIp === deviceProps.ip;

    return(
        <button className={`w-[90%] h-[40px] bg-[#303030] rounded-[5px] mt-[10px] center-v ${isSelected ? "border border-white" : ""}`}
        onClick={() => getDeviceIp(deviceProps.ip)}>
           <DynamicTitle title={deviceProps.properties[0][1]}/>
        </button>
    )
}

export default DevicesCard;