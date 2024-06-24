// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread::{self, spawn};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{scope::ipc::RemoteDomainAccessScope, Manager};
use tauri::{window, Window};
use tg_backend::journal::Journal;
use tokio::runtime::Runtime;

use tg_backend;
use uuid::uuid;

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
async fn new_journal(content: String, title: String) -> String {
    let j = Journal::new(title.clone());
    let id = match j {
    Ok(mut journal) => {
        journal.update_buffer(content);
        journal.update_buffer_title(title);
        match journal.write_to_disk() {
            Ok(_) => {},
            Err(e) => eprintln!("[NEW-JOURNAL][WRITE-TO-DISK] {e}"),
        }
        Some(journal.uuid_str)
    },
    Err(e) => {eprintln!("[TAURI-NEW-JOURNAL] {e}");None},
    };

    let id: String = match id {
    Some(uuid) => uuid,
    None => "".to_owned(),
    };
    id
    // we will check if the vec is the correct one on js side
}

#[tauri::command]
async fn update_buffer(content: String, title: String, id: String) {
    let uuid = uuid::Uuid::parse_str(&id).expect("uuid parsing failed");

    let j = Journal::init(uuid.as_bytes().to_vec());

    match j {
    Ok(mut journal) => { journal.update_buffer(content);
        journal.update_buffer_title(title);
        match journal.write_to_disk() {
            Ok(o) => {},
            Err(e) => eprintln!("[WRITE-TO-DISK] {e}"),
        }
     },
    Err(e) => eprintln!("[TAURI-UPDATE-BUFFER] {e}"),
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


#[tauri::command]
async fn createconfig() {
    let _ = tg_backend::config::Configuration::new();
}

#[tauri::command]
async fn importconfig() {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            opendialog,
            checkconfig,
            createconfig,
            importconfig,
            update_buffer,
            new_journal,
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
