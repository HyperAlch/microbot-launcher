// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::println;

use jagex_account::Account;

mod jagex_account;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
    let _docs_window = tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::External("https://account.jagex.com/oauth2/auth?auth_method=&login_type=&flow=launcher&response_type=code&client_id=com_jagex_auth_desktop_launcher&redirect_uri=https%3A%2F%2Fsecure.runescape.com%2Fm%3Dweblogin%2Flauncher-redirect&code_challenge=iXwr1ZtyblyEzY7mS1PVHy4MztmNiBbLwt8Qx0B5j5g&code_challenge_method=S256&prompt=login&scope=openid+offline+gamesso.token.create+user.profile.read&state=moq49__nfSgEX3x9xJlH2YWAG29A21knviv11Z1KoAg".parse().unwrap()),
    )
    .build()
    .unwrap();
}

#[tauri::command]
async fn jagex_login(_handle: tauri::AppHandle) {
    let mut account = Account::new();

    account.generate_login_url().await;

    println!("{}", account.login_url);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, open_docs, jagex_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
