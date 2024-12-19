use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_dxwebview);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Dxwebview<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.dxwebview", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_dxwebview)?;
    Ok(Dxwebview(handle))
}

/// Access to the dxwebview APIs.
pub struct Dxwebview<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Dxwebview<R> {
    pub fn create_webview(&self, payload: PingRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("createWebview", payload)
            .map_err(Into::into)
    }

    pub fn close_webview(&self, payload: PingRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("closeWebview", payload)
            .map_err(Into::into)
    }
}
