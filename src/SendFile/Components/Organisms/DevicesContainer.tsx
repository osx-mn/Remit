import DevicesCard from "../Molecules/DevicesCard";
import { useDevice } from "../../../context/DeviceContext";

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

    const { setDeviceSelected, deviceSelected, setDeviceSelectedIp }= useDevice();

    const getDeviceIp = (ip: string) => {
        console.log("ip presionado: ", ip);
        setDeviceSelectedIp(ip);
        setDeviceSelected(true);
    }

    return(
        /* móvil: ancho completo, altura automática mínima | PC: columna lateral fija, altura 100% */
        <div className="flex flex-col items-center w-full md:w-[250px] md:min-w-[250px] min-h-[120px] md:h-full bg-[#252525] rounded-[5px] p-2 overflow-y-auto shrink-0">
            <p className="text-white text-center text-[24px]">Dispositivos</p>
            {devicesList.map((device, _) => {
                return <DevicesCard 
                key={device.full_name}
                deviceProps={device}
                getDeviceIp={getDeviceIp}
                deviceCardSelected={deviceSelected}/>
            })}
        </div>
    )
}

export default DevicesContainer;