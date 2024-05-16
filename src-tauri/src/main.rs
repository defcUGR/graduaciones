// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::Ordering;

use tauri::WindowEvent;

#[macro_use]
extern crate log;

mod scanner;
mod session;
mod store;
mod ticket;

#[tauri::command]
fn create(email: String) -> () {
    ticket::create_ticket_pdf(ticket::create_ticket_data_from_email(email));
}

#[tauri::command]
fn jsatt() -> String {
    serde_json::to_string(&ticket::create_ticket_data_from_email(
        "deunaocampo@correo.ugr.es".into(),
    ))
    .unwrap()
}

fn main() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        // .format(|out, message, record| {
        //     out.finish(format_args!(
        //         "[{} {} {}] {}",
        //         humantime::format_rfc3339(std::time::SystemTime::now()),
        //         record.level(),
        //         record.target(),
        //         message
        //     ))
        // })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply()
        .unwrap();

    std::fs::create_dir_all(
        home::home_dir()
            .unwrap()
            .join(".graduaciones")
            .join("codes"),
    )
    .unwrap();

    tauri::Builder::default()
        .manage(session::SessionStore::new(
            home::home_dir()
                .unwrap()
                .join(".graduaciones")
                .join("sessions.json"),
        ))
        .invoke_handler(tauri::generate_handler![
            // scanner::get_serial_ports,
            scanner::start_scan,
            // session::get_sessions,
            // session::get_session_data,
            // session::create_session,
            create,
            jsatt
        ])
        .on_window_event(move |ev| match *ev.event() {
            WindowEvent::CloseRequested { .. } => scanner::EMIT_IDS.store(false, Ordering::SeqCst),
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
