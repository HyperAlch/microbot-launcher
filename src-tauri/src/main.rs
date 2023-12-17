// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::println;

use jagex_account::Account;
use tauri::Manager;
mod jagex_account;
mod user_agent;

#[tauri::command]
async fn jagex_login(handle: tauri::AppHandle, app_window: tauri::Window) {
    let mut account = Account::new();

    account.generate_login_url().await;

    let _popup_window = tauri::WindowBuilder::new(
        &handle,
        "login",
        tauri::WindowUrl::External(account.login_url.parse().unwrap()),
    )
    .on_navigation(move |url| {
        let url = url.as_str();

        if &url[0..10] == "jagex:code" {
            let _window = app_window.get_window("login").unwrap().close();
            println!("URL: {}", url);
            false
        } else {
            true
        }
    })
    .build()
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![jagex_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
