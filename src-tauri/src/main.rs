#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use std::io::Write;

#[tauri::command]
fn get() -> String {
    match serialport::available_ports() {
        Ok(infos) => {
            let mut port_infos = String::new();
            for info in infos {
                port_infos += &*(" ".to_owned() + &*info.port_name);
            }
            port_infos
        },
        Err(e) => { e.to_string() },
    }
}

#[tauri::command]
fn send(port: &str, data: &str) -> String {
    match serialport::new(port, 9600).open() {
        Ok(mut port) => {
            match port.write(data.as_bytes()) {
                Ok(_) => {
                    format!("Send success, data:{}",data)
                },
                Err(e) => { e.to_string() },
            }
        },
        Err(e) => { e.to_string() },
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
