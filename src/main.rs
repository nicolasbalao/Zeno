use std::collections::HashMap;

use crate::{hotkey::setup_hotkey, model::ApplicationInformation};

mod hotkey;
mod indexer;
mod launcher;
mod model;
mod search;

fn main() {
    let applications: HashMap<String, ApplicationInformation> = indexer::get_applications();
    run(&applications);
    // setup_hotkey(move || {
    //     run(&applications);
    // })
    // .unwrap()
}

fn run(applications: &HashMap<String, ApplicationInformation>) {
    println_applications(&applications);
    let mut app_name = user_io();

    let apps_name = applications.keys().cloned().collect::<Vec<String>>();

    let matches = search::search(&app_name, &apps_name);

    println!("Search results for '{}': {:?}", app_name, matches);

    app_name = if matches.is_empty() {
        println!("No matches found. Please enter a valid application name:");
        user_io()
    } else {
        matches[0].to_string()
    };

    if let Some(app) = applications.get(&app_name) {
        let child = launcher::launch_application(app);
        println!("Process started {:?}", child);
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

fn println_applications(applications: &HashMap<String, ApplicationInformation>) {
    for (name, _) in applications {
        println!("{}", name);
    }
}
