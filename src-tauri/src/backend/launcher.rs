use std::{
    io,
    path::Path,
    process::{Child, Command},
};

use crate::backend::model::ApplicationInformation;

pub fn launch_application(app: &ApplicationInformation) -> io::Result<Child> {
    let exec_path_str = Path::new(&app.exec_path).to_str().expect("Invalid path");

    let mut cmd = Command::new(exec_path_str);

    if let Some(args) = &app.cmd_args {
        cmd.args(args.split_ascii_whitespace());
    }

    if let Some(working_dir) = &app.working_dir {
        cmd.current_dir(working_dir);
    }

    cmd.spawn()
}
