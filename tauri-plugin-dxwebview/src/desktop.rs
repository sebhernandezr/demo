use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Dxwebview<R>> {
    Ok(Dxwebview(app.clone()))
}

/// Access to the dxwebview APIs.
pub struct Dxwebview<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Dxwebview<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn create_webview(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        let window = self.0.get_window("main").unwrap();
        window
            .add_child(
                tauri::WebviewBuilder::new(
                    "label",
                    tauri::WebviewUrl::External(payload.value.clone().unwrap().parse().unwrap()),
                )
                .auto_resize(),
                tauri::PhysicalPosition::new(0, 0),
                tauri::PhysicalSize::new(
                    window.inner_size().unwrap().width / 2,
                    window.inner_size().unwrap().height,
                ),
            )
            .unwrap();

        Ok(PingResponse {
            value: payload.value,
        })
    }
}
