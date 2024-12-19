use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::DxwebviewExt;
use crate::Result;

#[command]
pub(crate) async fn create_webview<R: Runtime>(
    app: AppHandle<R>,
    payload: DxWebviewRequest,
) -> Result<()> {
    app.dxwebview().create_webview(payload)
}

#[command]
pub(crate) async fn close_webview<R: Runtime>(
    app: AppHandle<R>,
    payload: DxWebviewRequest,
) -> Result<()> {
    app.dxwebview().close_webview(payload)
}
