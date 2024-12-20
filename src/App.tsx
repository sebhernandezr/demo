import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import {useState} from "react";

function App() {
    const [open, setOpen] = useState(false);

    return (
        <main className={open ? "split" : "full"}>
            <h1>POC split screen Android</h1>
            <button onClick={async () => {
                await invoke("plugin:dxwebview|create_webview", {
                    payload: {
                        url: "https://www.digiexam.com/",
                        label: "test"
                    }
                });
                setOpen(true);
            }}>add
            </button>
            <button onClick={async () => {
                await invoke("plugin:dxwebview|close_webview", {
                    payload: {
                        url: "https://www.digiexam.com/",
                        label: "test"
                    }
                });
                setOpen(false);
            }}>close
            </button>
            <button onClick={async () => {
                console.log(await invoke("plugin:webview-manager|ping", {
                    payload: {
                        value: "test"
                    }
                }))
            }}>ping
            </button>
        </main>
    );
}

export default App;
