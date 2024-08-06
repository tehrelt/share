#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod server;
use std::{str::Bytes, sync::Mutex, thread};

use server::server::TcpServer;
use tauri::{Manager, Runtime};

struct AppState {
    server: TcpServer,
}

impl AppState {
    pub fn new(server: TcpServer) -> Self {
        Self { server }
    }
}

#[tauri::command]
async fn upload_file<R: Runtime>(path: String, app: tauri::AppHandle<R>) -> Result<String, String> {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();
    state.server.upload_file(path.to_owned());
    Ok(path.to_owned())
}

#[tauri::command]
fn gen_qr() -> Result<Bytes<'static>, String> {
    Ok(())
}

fn main() {
    let port = 7878;
    let server = TcpServer::new("0.0.0.0".to_string(), port);

    tauri::Builder::default()
        .setup(|app| {
            let state = Mutex::new(AppState::new(server));

            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
