// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, State,
};

use crate::backend::{
    indexer::get_applications, launcher::launch_application, model::ApplicationInformation,
    search::search,
};

mod backend;

#[derive(Debug, Default)]
struct AppState {
    applications: Vec<ApplicationInformation>,
}

#[tauri::command]
fn get_applications_names(state: State<'_, Mutex<AppState>>) -> Vec<ApplicationInformation> {
    let applications = get_applications();

    let applications = applications
        .values()
        .into_iter()
        .map(|app| app.clone())
        .collect::<Vec<ApplicationInformation>>();

    {
        let mut app_state = state.lock().unwrap();
        app_state.applications = applications.clone();
    }

    applications
}

#[tauri::command]
fn search_application(
    name: &str,
    state: State<'_, Mutex<AppState>>,
) -> Vec<ApplicationInformation> {
    let app_state = state.lock().unwrap();
    let applications: Vec<ApplicationInformation> = app_state
        .applications
        .iter()
        .map(|app| app.clone())
        .collect();

    // drop(app_state);

    if name.is_empty() {
        return applications.clone();
    }

    search(name, applications)
}

#[tauri::command]
fn launch_application_cmd(name: &str) {
    let applications = get_applications();
    let application = applications.get(name).unwrap();

    launch_application(application).unwrap();
}

fn show_main_window(app: AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn hide_main_window(app: AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                let escp_shortcut = Shortcut::new(None, Code::Escape);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, _| match shortcut {
                            s if s == &ctrl_n_shortcut => {
                                let app_handle = _app.app_handle().clone();

                                show_main_window(app_handle);
                            }
                            s if s == &escp_shortcut => {
                                let app_handle = _app.app_handle().app_handle().clone();
                                hide_main_window(app_handle);
                            }
                            _ => {
                                println!("Unhandled")
                            }
                        })
                        .build(),
                )?;
                app.global_shortcut().register(ctrl_n_shortcut)?;
                app.global_shortcut().register(escp_shortcut)?;
            }

            // Sys tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_applications_names,
            search_application,
            launch_application_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
