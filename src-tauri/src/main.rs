// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::exit;
use std::thread::{self, spawn};
use std::time::Duration;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{scope::ipc::RemoteDomainAccessScope, Manager};
use tauri::{window, Window};
use tg_backend::journal::{Journal, Media};
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
    tg_backend::config::Configuration::exists()
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
                Ok(_) => {}
                Err(e) => eprintln!("[NEW-JOURNAL][WRITE-TO-DISK] {e}"),
            }
            Some(journal.uuid_str)
        }
        Err(e) => {
            eprintln!("[TAURI-NEW-JOURNAL] {e}");
            None
        }
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
        Ok(mut journal) => {
            journal.update_buffer(content);
            journal.update_buffer_title(title);
            match journal.write_to_disk() {
                Ok(_) => {}
                Err(e) => eprintln!("[WRITE-TO-DISK] {e}"),
            }
        }
        Err(e) => eprintln!("[TAURI-UPDATE-BUFFER] {e}"),
    }
}

#[tauri::command]
async fn call_js(function: String, args: String, app: tauri::AppHandle) {
    if let Some(main_win) = app.get_window("main") {
        let _ = main_win.eval(&format!("{function}({args});"));
        println!("[TG-BACKEND] evaluating {function}({args})")
    }
}

#[tauri::command]
async fn createconfig(app: tauri::AppHandle) {
    match tg_backend::config::Configuration::create() {
        Ok(_) => {
            if let Some(main_win) = app.get_window("main") {
                let _ = main_win.eval(&format!(
                    "window.location.replace('http://localhost:{}/{}')",
                    "3000", "create"
                ));
                println!("[TG-BACKEND](create-config) redirected to create")
            }
        }
        Err(e) => eprintln!("creating config failed: {e}"),
    }
}

#[tauri::command]
async fn fill_post(postId: String, app: tauri::AppHandle) {
    let uuid = uuid::Uuid::parse_str(&postId).expect("uuid parsing failed");

    let j = match Journal::init(uuid.into_bytes().to_vec()) {
        Ok(o) => {println!("\x1b[92m{:#?}\x1b[0m", o);o},
        Err(e) => {
            eprintln!("[TG-BACKEND](FILL-POST) {e}");
            exit(1);
        }
    };

    if let Some(main_win) = app.get_window("main") {
        // go to create page
        // activate the listener for fill_post
        match main_win.eval(&format!("document.querySelector('#two').click();loadlisteners();")) {
            Ok(_) => {println!("\x1b[32m[TG-BACKEND](FILL-POST)\x1b[0m clicked create, activated listener")},
            Err(e) => {
                eprintln!("\x1b[31m[TG-BACKEND](FILL-POST)\x1b[0m {e}")
            },
        }
        
        println!("\x1b[32m[TG-BACKEND](FILL-POST)\x1b[0m redirected to create")
    }

    tokio::time::sleep(Duration::from_millis(300)).await;

    match app.emit_to("main", "fill_post", j) {
    Err(e) => eprintln!("\x1b[31m[TG-BACKEND] (fill-post)\x1b[0m{e}"),
    Ok(_) => {println!("\x1b[32m[TG-BACKEND](FILL-POST)\x1b[0m successfully emited 'fill_post'")},
    }
    let wallp = match Media::get(uuid.as_bytes().to_vec(), tg_backend::journal::MediaType::Wallpaper(String::new())) {
    tg_backend::journal::MediaType::Wallpaper(w) => w,
    _ => "unreachable".into()
    };

    println!("wallpaper: {wallp}");
        
    

    match app.emit_to("main", "change_style", wallp) {
    Err(e) => eprintln!("\x1b[31m[TG-BACKEND] (fill-post)\x1b[0m{e}"),
    Ok(_) => {println!("\x1b[32m[TG-BACKEND](FILL-POST)\x1b[0m successfully emited 'change_style'")},
    }

    // redirect to create
    // change wallpaper, font
    // fill date, title, and content
    // make it look nicer
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            opendialog,
            checkconfig,
            createconfig,
            update_buffer,
            new_journal,
            start_backend,
            redirect,
            call_js,
            fill_post,
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
