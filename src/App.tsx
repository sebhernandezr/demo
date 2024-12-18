import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import {useState} from "react";

function App() {
    const [open, setOpen] = useState(false);

    async function addWebview() {
        setOpen(true);
        await invoke("plugin:dxwebview|create_webview", {payload: {value: "https://www.geogebra.org/classic"}});
    }

    async function removeWebview() {
        setOpen(false);
        // await invoke("plugin:dxwebview|remove_webview", {payload: {value: "https://www.geogebra.org/classic"}});
    }

    return (
        <main className={open ? "split" : "full"}>
            <h1>POC split screen Android</h1>
            <button onClick={() => addWebview()}>add</button>
            <button onClick={() => removeWebview()}>remove</button>
        </main>
    );
}

export default App;
