#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Définition globale pour l'état du chronomètre.
static IS_RUNNING: Lazy<Arc<Mutex<bool>>> = 
    Lazy::new(|| Arc::new(Mutex::new(false)));
static ELAPSED_TIME: Lazy<Arc<Mutex<Duration>>> =
    Lazy::new(|| Arc::new(Mutex::new(Duration::new(0, 0))));
static START_TIME: Lazy<Arc<Mutex<Instant>>> = 
    Lazy::new(|| Arc::new(Mutex::new(Instant::now())));

fn main() {
    let (tx, rx) = mpsc::channel();

    tauri::Builder::default()
        .setup(move |app| {
            let tx_clone = tx.clone();
            let is_running_clone = IS_RUNNING.clone();

            thread::spawn(move || {
                loop {
                    let elapsed_time;
                    {
                        let is_running = is_running_clone.lock().unwrap();
                        let start_time = START_TIME.lock().unwrap();

                        if *is_running {
                            // If the chronometer is running, calculate the time since the last start_time.
                            elapsed_time = *ELAPSED_TIME.lock().unwrap() + start_time.elapsed();
                            // it updates the time
                        } else {
                            // If the chronometer is not running, use the recorded elapsed time.
                            elapsed_time = *ELAPSED_TIME.lock().unwrap();
                            // it doesn't updates the time
                        }
                    }

                    // Format and send the elapsed time.
                    let time = format_duration(elapsed_time);
                    println!("time: {:?}", &time);
                    // println!("is_running: {:?}", &IS_RUNNING);
                    // println!("ELAPSED_TIME: {:?}", &ELAPSED_TIME);
                    // println!("START_TIME: {:?}", &START_TIME);
                    tx_clone.send(time).expect("Failed to send time");

                    thread::sleep(Duration::from_millis(1));
                }
            });

            // Here you can use app.handle to get a handle that is 'static and can be sent across threads
            let app_handle = app.handle();
            thread::spawn(move || {
                for time in rx {
                    app_handle
                        .emit_all("chronometer-update", Payload { message: time })
                        .expect("Failed to emit event");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            toggle_chronometer,
            reset_chronometer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}

#[tauri::command]
fn toggle_chronometer() {
    let mut is_running = IS_RUNNING.lock().unwrap();
    let mut elapsed = ELAPSED_TIME.lock().unwrap();
    let mut start_time = START_TIME.lock().unwrap();

    if *is_running {
        // If the chronometer is currently running, we're about to pause it.
        // Record the current elapsed time.
        *elapsed += start_time.elapsed();
       
    }
    // Reset the start time to now to begin counting from this moment upon resume.
    *start_time = Instant::now();

    *is_running = !*is_running;
}

#[tauri::command]
fn reset_chronometer() {
    //let mut is_running = IS_RUNNING.lock().unwrap();
    let mut elapsed = ELAPSED_TIME.lock().unwrap();
    let mut start_time = START_TIME.lock().unwrap();

    *elapsed = Duration::new(0, 0);
    *start_time = Instant::now();
    //*is_running = false;
}
