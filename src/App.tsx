import {invoke} from "@tauri-apps/api/core";
import "./App.css";

function App() {
    return (
        <button onClick={async () => {
            await invoke("create_webview", {label: "geogebra", url: "https://www.geogebra.org/classic"});
            console.log(await invoke("plugin:dxwebview|create_webview", {payload: {value: "https://www.geogebra.org/classic"}}));
        }}>
            Greet
        </button>
    );
}

export default App;
