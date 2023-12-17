// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use jagex_account::generate_login_url;
use std::println;

use tauri::Manager;

use crate::jagex_account::{get_login_data, AddingAccountPayload};
mod jagex_account;
mod user_agent;

#[tauri::command]
async fn jagex_login(handle: tauri::AppHandle, app_window: tauri::Window) {
    let login_url = generate_login_url().await;
    let user_agent = user_agent::generate_user_agent();

    let _popup_window = tauri::WindowBuilder::new(
        &handle,
        "login",
        tauri::WindowUrl::External(login_url.parse().unwrap()),
    )
    .user_agent(&user_agent)
    .on_navigation(move |url| {
        let url = url.to_string();

        if &url[0..10] == "jagex:code" {
            let _window = app_window.get_window("login").unwrap().close();

            let _result = app_window.emit_all(
                "adding_account",
                AddingAccountPayload {
                    url,
                    user_agent: user_agent.clone(),
                },
            );
            false
        } else {
            true
        }
    })
    .build()
    .unwrap();
}

#[tauri::command]
async fn get_jagex_login_data(payload: AddingAccountPayload) {
    get_login_data(payload.url).await;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![jagex_login, get_jagex_login_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, format, println};

    #[test]
    fn it_works() {
        let mut params = "jagex:code=FdNTGX25XvjSY6ZDgSgewInbv5aHcpftsC57xJmja20.9Ims9sFbVACYJONx0HkzjI5ng6uvgXY0jhqZivYvn44,state=ITCgEOMKA..Myy.2O51nU9UcfW~HAyc9x-2hmXGN1bw,intent=social_auth".to_string();

        params = params.replace("jagex:", "");
        let params: Vec<(String, String)> = params
            .split(",")
            .map(|x| x.to_string())
            .map(|x| {
                let split: Vec<&str> = x.split("=").collect();
                (
                    split
                        .get(0)
                        .expect(&format!("Failed parsing code, state, or intent: {}", x))
                        .to_string(),
                    split
                        .get(1)
                        .expect(&format!("Failed parsing code, state, or intent: {}", x))
                        .to_string(),
                )
            })
            .collect();

        let mut param_dict: HashMap<String, String> = Default::default();
        for param in params {
            param_dict.insert(param.0, param.1);
        }

        let code = param_dict.get("code").expect("Failed to parse code");
        let state = param_dict.get("state").expect("Failed to parse state");

        println!("Code: {:?}", code);
        println!("State: {:?}", state);
    }
}
