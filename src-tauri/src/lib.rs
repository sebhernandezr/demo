mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            init_desktop(app)?;

            #[cfg(mobile)]
            init_mobile(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_dxwebview::init())
        .invoke_handler(tauri::generate_handler![commands::create_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(desktop)]
pub fn init_desktop(app: &mut tauri::App) -> Result<(), String> {
    tauri::async_runtime::block_on(async move {
        let window = tauri::WindowBuilder::new(app, "main")
            .inner_size(800., 600.)
            .title("demo")
            .build()
            .unwrap();

        let webview_builder = tauri::WebviewBuilder::new(
            "main",
            tauri::WebviewUrl::App("index.html".parse().unwrap()),
        )
        .auto_resize();

        window
            .add_child(
                webview_builder,
                tauri::PhysicalPosition::new(0, 0),
                window.inner_size().unwrap(),
            )
            .unwrap();

        Ok(())
    })
}

#[cfg(mobile)]
pub fn init_mobile(app: &mut tauri::App) -> Result<(), String> {
    tauri::async_runtime::block_on(async move {
        let _window = tauri::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::App("index.html".parse().unwrap()),
        )
        .build()
        .unwrap();

        Ok(())
    })
}
