use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

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
        // window
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
