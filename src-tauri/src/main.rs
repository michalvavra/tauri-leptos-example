#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct GreetEvent {
    greeting: String,
}

#[derive(Clone, serde::Serialize)]
struct GenericEvent {
    num: u16,
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(window: Window, name: &str) -> String {
    // Events should be emitted from `Window` object only.
    // `tauri-sys` expects window name to be part of event payload.
    window
        .emit_all(
            "greet-event",
            GreetEvent {
                greeting: format!("Hey {}, this message has been sent from Tauri event.", name),
            },
        )
        .unwrap();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn emit_event(window: Window, num: u16) {
    window
        .emit_all(
            "generic-event",
            GenericEvent {
                num,
                message: format!("Generic event with number {}", num),
            },
        )
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, emit_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
