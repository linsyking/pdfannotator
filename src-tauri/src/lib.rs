// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use tauri::ipc::Response;
use tauri::Manager;
use tauri::State;
use tauri_plugin_cli::CliExt;

use serde_json::{Value, to_string_pretty};

fn prettify_json(json_str: String) -> String {
    match serde_json::from_str::<Value>(&json_str) {
        Ok(parsed) => to_string_pretty(&parsed).unwrap_or_else(|_| "[]".to_string()),
        Err(_) => "[]".to_string(),
    }
}


#[derive(Default)]
struct AppState {
    filepath: String,
}

#[tauri::command]
fn getconfig(state: State<AppState>) -> Result<String, ()> {
    let fp = state.filepath.clone();
    let data = std::fs::read(fp + ".json").map_err(|_| ())?;
    let data = String::from_utf8(data).unwrap();
    // data
    Ok(data)
}

#[tauri::command]
fn save(state: State<AppState>, data: String) -> String {
    let fp = state.filepath.clone();
    // Format data (JSON) to pretty string
    let pretty_data = prettify_json(data);
    // Write to file
    let res = std::fs::write(fp + ".json", pretty_data);
    if let Err(e) = res {
        println!("Error: {}", e);
    }
    String::new()
}

#[tauri::command]
fn file(state: State<AppState>) -> Response {
    let fp = state.filepath.clone();
    let data = std::fs::read(fp).unwrap();
    tauri::ipc::Response::new(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            let mut valid = false;
            match app.cli().matches() {
                Ok(matches) => {
                    // println!("{:?}", matches.args);
                    let arg = matches.args.get("source").unwrap();
                    match &arg.value {
                        Value::String(value) => {
                            // Check if file exists
                            if !std::path::Path::new(value).exists() {
                                return Err("File does not exist".into());
                            }
                            app.manage(AppState {
                                filepath: value.clone(),
                            });
                            valid = true;
                        }
                        _ => {}
                    }
                }
                Err(_) => {}
            }
            if valid {
                Ok(())
            } else {
                Err("Invalid arguments".into())
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![getconfig, file, save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
