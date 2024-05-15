// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::Ordering;

use tauri::WindowEvent;

mod scanner;
mod session;
mod store;
mod ticket;

#[tauri::command]
pub fn create(email: String) -> String {
    ticket::create_ticket_from_email(email)
}

fn main() {
    std::fs::create_dir_all(home::home_dir().unwrap().join(".graduaciones")).unwrap();

    tauri::Builder::default()
        .manage(session::SessionStore::new(
            home::home_dir()
                .unwrap()
                .join(".graduaciones")
                .join("sessions.json"),
        ))
        .invoke_handler(tauri::generate_handler![
            // scanner::get_serial_ports,
            // scanner::start_scan,
            // session::get_sessions,
            // session::get_session_data,
            // session::create_session,
            create
        ])
        .on_window_event(move |ev| match *ev.event() {
            WindowEvent::CloseRequested { .. } => scanner::EMIT_IDS.store(false, Ordering::SeqCst),
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
