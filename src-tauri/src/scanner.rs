use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serialport::{SerialPort, SerialPortInfo};
use tauri::Manager;

pub static EMIT_IDS: AtomicBool = AtomicBool::new(true);
static mut SCAN_HANDLE: Mutex<Option<JoinHandle<Result<(), String>>>> = Mutex::new(None);

#[tauri::command]
pub fn start_scan(app_handle: tauri::AppHandle, port_name: &str) -> Result<(), String> {
    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .map_err(|e| e.to_string())?;

    if unsafe { SCAN_HANDLE.lock().map_err(|e| e.to_string())?.is_some() } {
        return Err("scan already running".to_owned());
    }
    EMIT_IDS.store(true, Ordering::SeqCst);
    start_scanner(app_handle, port).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    serialport::available_ports().map_err(|e| e.to_string())
}

fn start_scanner(
    handle: tauri::AppHandle,
    mut port: Box<dyn SerialPort>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("called scan to start");
    unsafe {
        *(SCAN_HANDLE.lock().unwrap()) = Some(thread::spawn(move || {
            let close_listen_id = handle.listen_global("close_scan", |_| {
                EMIT_IDS.store(false, Ordering::SeqCst);
            });

            let mut serial_buf: Vec<u8> = vec![0; 199];
            while EMIT_IDS.load(Ordering::SeqCst) {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(_) => {
                        println!("Scanned: {:?}", (serial_buf.clone()));
                        handle
                            .emit_all("id_scanned", String::from_utf8(serial_buf.clone()).unwrap())
                            .map_err(|e: tauri::Error| e.to_string())
                            .unwrap()
                    }
                    Err(_) => {}
                }
            }

            std::mem::drop(port);
            handle.unlisten(close_listen_id);
            handle
                .emit_all("scan_closed", true)
                .map_err(|e| e.to_string())
                .unwrap();
            *(SCAN_HANDLE.lock().unwrap()) = None;

            Ok(())
        }));
    }

    Ok(())
}
