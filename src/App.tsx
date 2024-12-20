import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import {useState} from "react";

function App() {
    const [open, setOpen] = useState(false);
    const [res, setRes] = useState<{ value?: string }>({value: ""});

    return (
        <main className={open ? "split" : "full"}>
            <h1>POC webview mobile</h1>
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
                let res = await invoke<{ value?: string }>("plugin:webview-manager|ping", {
                    payload: {
                        value: "pong"
                    }
                })
                setRes(res);
            }}>ping
            </button>
            {res.value}
        </main>
    );
}

export default App;
