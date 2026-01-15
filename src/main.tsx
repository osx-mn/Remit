import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

import ReactDOM from "react-dom/client";
import TitleBar from "./TitleBar/Pages/TitleBar";
import FilePage from "./SendFile/Pages/FilePage";
import "./index.css";

const App = () =>{
  const [showTitlebar, setShowTitlebar] = useState<boolean>(true);

  const GetOs = async () =>{
    let showTitlebarInSpecificOS = await invoke<boolean>("show_custom_titlebar_in_os");
    setShowTitlebar(showTitlebarInSpecificOS);
  }

  useEffect(() => {
    GetOs
  }, []);

  return(
    <>
      {showTitlebar? <TitleBar /> : null}
      <FilePage />
    </>
  )
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <>
    <App />
  </>
);