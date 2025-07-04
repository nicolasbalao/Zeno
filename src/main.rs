use std::{collections::HashMap, path::Path};

use crate::hotkey::setup_hotkey;

mod hotkey;
mod indexer;
mod launcher;

fn main() {
    let applications: HashMap<String, String> = indexer::get_applications();
    // run(&applications);
    setup_hotkey(move || {
        run(&applications);
    })
    .unwrap()
}

fn run(applications: &HashMap<String, String>) {
    println_applications(&applications);
    let app_name = user_io();

    if let Some(exec_path) = applications.get(&app_name) {
        launcher::launch_application(Path::new(exec_path));
    } else {
        eprintln!("Application not found: {}", app_name);
    }
}

fn user_io() -> String {
    let stdin = std::io::stdin();
    println!("\nEnter the name of the application to launch:");
    let mut app_name = String::new();
    stdin.read_line(&mut app_name).expect("Failed to read line");
    app_name.trim().to_string()
}

fn println_applications(applications: &HashMap<String, String>) {
    for (name, _) in applications {
        println!("{}", name);
    }
}
