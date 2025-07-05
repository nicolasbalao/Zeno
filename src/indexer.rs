use std::{collections::HashMap, path::Path};

use parselnk::Lnk;
use walkdir::{DirEntry, WalkDir};

use crate::model::ApplicationInformation;

pub fn get_applications() -> std::collections::HashMap<String, ApplicationInformation> {
    let mut applications: HashMap<String, ApplicationInformation> = HashMap::new();
    let global_applications = get_global_start_menu_applications();
    let user_applications = get_user_start_menu_applications();

    // Merge global applications first
    applications.extend(global_applications);

    // Then merge user applications (will overwrite duplicates)
    applications.extend(user_applications);

    applications
}

fn get_global_start_menu_applications() -> std::collections::HashMap<String, ApplicationInformation>
{
    let global_start_menu_path =
        Path::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs");
    extract_application_data(global_start_menu_path)
}

fn get_user_start_menu_applications() -> HashMap<String, ApplicationInformation> {
    // Get the current user's username from the USERNAME environment variable
    if let Ok(username) = std::env::var("USERNAME") {
        let user_path = format!(
            "C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs",
            username
        );
        let user_start_menu_path = Path::new(&user_path);
        extract_application_data(user_start_menu_path)
    } else {
        // If USERNAME env var is not available, return empty HashMap
        HashMap::new()
    }
}

fn extract_application_data(path: &Path) -> HashMap<String, ApplicationInformation> {
    let lnk_files: Vec<DirEntry> = WalkDir::new(path)
        .into_iter()
        .filter(|e| is_lnk_file(e.as_ref().unwrap()))
        .map(|e| e.unwrap())
        .collect();

    lnk_files
        .iter()
        .filter_map(|file| {
            let lnk = Lnk::try_from(file.path()).ok()?;
            if file.file_name().to_str().unwrap().contains("Discord") {
                println!("{:?}", lnk.string_data)
            }

            let exec_path = lnk.link_info.local_base_path?;
            let application_name = file.path().file_stem()?.to_str().unwrap().to_string();
            let working_dir = lnk.string_data.working_dir;
            let cmd_args = lnk.string_data.command_line_arguments;

            let app = ApplicationInformation {
                name: application_name,
                exec_path,
                cmd_args,
                working_dir,
            };
            Some((app.name.clone(), app))
        })
        .collect()
}

fn is_lnk_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with(".lnk")
}
