use std::{path::Path, process::Command};

pub fn launch_application(exec_path: &Path) {
    let exec_path_str = exec_path.to_str().expect("Invalid path");

    Command::new(exec_path_str)
        .spawn()
        .expect("Failed to launch application");
}
