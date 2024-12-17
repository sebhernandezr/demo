#[tauri::command]
pub fn create_webview(window: tauri::Window, label: &str, url: &str) -> Result<(), String> {
    #[cfg(desktop)]
    window
        .add_child(
            tauri::WebviewBuilder::new(label, tauri::WebviewUrl::External(url.parse().unwrap()))
                .auto_resize(),
            tauri::PhysicalPosition::new(0, 0),
            tauri::PhysicalSize::new(
                window.inner_size().unwrap().width / 2,
                window.inner_size().unwrap().height,
            ),
        )
        .unwrap();

    Ok(())
}
