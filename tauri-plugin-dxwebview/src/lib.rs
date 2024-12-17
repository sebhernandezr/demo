use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Dxwebview;
#[cfg(mobile)]
use mobile::Dxwebview;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the dxwebview APIs.
pub trait DxwebviewExt<R: Runtime> {
    fn dxwebview(&self) -> &Dxwebview<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DxwebviewExt<R> for T {
    fn dxwebview(&self) -> &Dxwebview<R> {
        self.state::<Dxwebview<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("dxwebview")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::create_webview
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let dxwebview = mobile::init(app, api)?;
            #[cfg(desktop)]
            let dxwebview = desktop::init(app, api)?;
            app.manage(dxwebview);
            Ok(())
        })
        .build()
}
