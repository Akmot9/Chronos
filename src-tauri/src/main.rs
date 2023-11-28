// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::{time::{Instant, Duration}, thread, sync::mpsc};
use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}
fn main() {
    let (tx, rx) = mpsc::channel();

    tauri::Builder::default()
        .setup(move |app| {
            let start = Instant::now();
            let tx_clone = tx.clone();

            thread::spawn(move || {
                loop {
                    let duration = start.elapsed();
                    let total_seconds = duration.as_secs();
                    let hours = total_seconds / 3600;
                    let minutes = (total_seconds % 3600) / 60;
                    let seconds = total_seconds % 60;
                    let milliseconds = duration.subsec_millis();

                    let time = format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds);
                    tx_clone.send(time).expect("Failed to send time");
                    thread::sleep(Duration::from_millis(1));
                }
            });

            // Here you can use app.handle to get a handle that is 'static and can be sent across threads
            let app_handle = app.handle();
            thread::spawn(move || {
                for time in rx {
                    app_handle.emit_all("event-name", Payload { message: time }).expect("Failed to emit event");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



