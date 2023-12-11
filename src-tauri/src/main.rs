#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::sync::Mutex;
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

struct ChronometerState {
    is_running: Mutex<bool>,
    elapsed_time: Mutex<Duration>,
    start_time: Mutex<Instant>,
    lap_times: Mutex<Vec<Duration>>,
}
    
fn main() {
    let (tx, rx) = mpsc::channel();

    tauri::Builder::default()
        .setup(move |app| {

            let chronometer_state = ChronometerState {
                is_running: Mutex::new(false),
                elapsed_time: Mutex::new(Duration::new(0, 0)),
                start_time: Mutex::new(Instant::now()),
                lap_times: Mutex::new(Vec::new()),
            };

            app.manage(chronometer_state);

            let tx_clone = tx.clone();
            let app_handle = app.handle();

            thread::spawn(move || {
                loop {
                    let elapsed_time;
                    {
                        let state = app_handle.state::<ChronometerState>();
                        let is_running = state.is_running.lock().unwrap();
                        let start_time = state.start_time.lock().unwrap();

                        if *is_running {
                            // If the chronometer is running, calculate the time since the last start_time.
                            elapsed_time = *state.elapsed_time.lock().unwrap() + start_time.elapsed();
                            // it updates the time
                        } else {
                            // If the chronometer is not running, use the recorded elapsed time.
                            elapsed_time = *state.elapsed_time.lock().unwrap()
                            // it doesn't updates the time
                        }
                    }

                    // Format and send the elapsed time.
                    let time = format_duration(elapsed_time);
                    //println!("time: {:?}", &time);
                    // println!("is_running: {:?}", &IS_RUNNING);
                    // println!("ELAPSED_TIME: {:?}", &ELAPSED_TIME);
                    // println!("START_TIME: {:?}", &START_TIME);
                    tx_clone.send(time).expect("Failed to send time");

                    thread::sleep(Duration::from_millis(10));
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
            reset_chronometer,
            save_lap
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
fn toggle_chronometer(state: tauri::State<ChronometerState>) {
    let mut is_running = state.is_running.lock().unwrap();
    let mut elapsed = state.elapsed_time.lock().unwrap();
    let mut start_time = state.start_time.lock().unwrap();

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
fn reset_chronometer(state: tauri::State<ChronometerState>) {
    //let mut is_running = IS_RUNNING.lock().unwrap();
    let mut elapsed = state.elapsed_time.lock().unwrap();
    let mut start_time = state.start_time.lock().unwrap();
    let mut lap_times = state.lap_times.lock().unwrap();

    *elapsed = Duration::new(0, 0);
    *start_time = Instant::now();
    lap_times.clear();
    //*is_running = false;
}

#[tauri::command]
fn save_lap(state: tauri::State<ChronometerState>) -> String {
    let mut lap_times = state.lap_times.lock().unwrap();
    let elapsed = state.elapsed_time.lock().unwrap();
    let start_time = state.start_time.lock().unwrap();

    let current_time = if *state.is_running.lock().unwrap() {
        *elapsed + start_time.elapsed()
    } else {
        *elapsed
    };

    // Compute lap duration
    let lap_duration = if let Some(last_lap) = lap_times.last().cloned() {
        current_time - last_lap
    } else {
        current_time
    };

    // Push the current time (not the lap duration) to lap_times for next calculation
    lap_times.push(current_time);

    // Return formatted lap duration
    format_duration(lap_duration)
}
