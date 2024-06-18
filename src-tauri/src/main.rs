// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread::{self, spawn};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{scope::ipc::RemoteDomainAccessScope, Manager};
use tauri::{window, Window};
use tokio::runtime::Runtime;

use tg_backend;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn opendialog() {
    FileDialogBuilder::new().pick_folder(|folder_path| {
        if let Some(path) = folder_path {
            println!("{}", path.display());
        } else {
            println!("Go back!");
        }
    });
}

#[tauri::command]
async fn start_backend(app: tauri::AppHandle) -> Result<(), ()> {
    // if let Some(win) = get_main_window(app).await {
    //     win.emit("redirect", "intro")
    //         .expect("failed to redirect to /intro");
    // }
    tg_backend::start().await;

    Ok(())
}

#[tauri::command]
async fn checkconfig(app: tauri::AppHandle) -> bool {
    let configfound = false;
    configfound
    // if let Some(main_win) = app.get_window("main") {
    //     if !configfound {
    //         let to = "intro";
    //         let _ = main_win.eval(&format!(
    //             "window.location.replace('http://localhost:{}/{}')",
    //             "3000", to
    //         ));
    //         println!("[TG-BACKEND] redirected to {}", to)
    //     } else {
    //         let to = "create";
    //         let _ = main_win.eval(&format!(
    //             "window.location.replace('http://localhost:{}/{}')",
    //             "3000", to
    //         ));
    //         println!("[TG-BACKEND] redirected to {}", to)
    //     }
    // }

}


#[tauri::command]
async fn redirect(to: String, app: tauri::AppHandle) {
    if let Some(main_win) = app.get_window("main") {
        let _ = main_win.eval(&format!(
            "window.location.replace('http://localhost:{}/{}')",
            "3000", to
        ));
        println!("[TG-BACKEND] redirected to {}", to)
    }
}

#[tauri::command]
async fn call_js(function: String, args: String, app: tauri::AppHandle) {
    if let Some(main_win) = app.get_window("main") {
        let _ = main_win.eval(&format!(
            "{function}({args});"));
        println!("[TG-BACKEND] evaluating {function}({args})")
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            opendialog,
            checkconfig,
            start_backend,
            redirect,
            call_js
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_main_window(app: tauri::AppHandle) -> Option<Window> {
    if let Some(main) = app.get_window("main") {
        return Some(main);
    }
    None
}
