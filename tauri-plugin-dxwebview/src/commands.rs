use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::DxwebviewExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.dxwebview().ping(payload)
}

#[command]
pub(crate) async fn create_webview<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.dxwebview().create_webview(payload)
}
