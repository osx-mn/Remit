import { getCurrentWindow } from "@tauri-apps/api/window";

import CloseButton from "../Components/Atoms/CloseButton";
import MinimizeButton from "../Components/Atoms/MinimizeButton";
import MaximizeButton from "../Components/Atoms/MaximizeButton";

import { CloseIcon, MinimizeIcon, MaximizeIcon } from "../TitleBarIcons/TitleBarIcons";

const TitleBar: React.FC = () => {
    const appwindow = getCurrentWindow();

    const minimize = () => { appwindow.minimize(); }
    const maximize = () => { appwindow.toggleMaximize(); }
    const close = () => { appwindow.close(); }
    return(
        <div className="h-[35px] flex justify-end w-full bg-[#1d1d1d] TitleBarDragAndDrop z-999">
            <MinimizeButton btnIcon={MinimizeIcon} onClick={minimize}/>
            <MaximizeButton btnIcon={MaximizeIcon} onClick={maximize}/>
            <CloseButton btnIcon={CloseIcon} onClick={close}/>            
        </div>
    );
}

export default TitleBar;