// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

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
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            println!("{:?}", shortcut);

                            if shortcut == &ctrl_n_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        println!("CTRL + N pressed");
                                    }
                                    ShortcutState::Released => {
                                        println!("CTRL + N Released !");
                                    }
                                }
                            }
                        })
                        .build(),
                )?;
                app.global_shortcut().register(ctrl_n_shortcut)?;
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
                        println!("quit menu item was clicked");
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
