// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::backend::{indexer::get_applications, launcher::launch_application, search::search};

mod backend;

#[tauri::command]
fn get_applications_names() -> Vec<String> {
    let applications = get_applications();

    applications
        .keys()
        .into_iter()
        .map(|app| app.clone())
        .collect::<Vec<String>>()
}

#[tauri::command]
fn search_application(name: &str, application_names: Vec<String>) -> Vec<String> {
    search(name, &application_names)
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

#[tauri::command]
fn launch_application_cmd(name: &str) {
    let applications = get_applications();
    let application = applications.get(name).unwrap();

    launch_application(application).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_applications_names,
            search_application,
            launch_application_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
