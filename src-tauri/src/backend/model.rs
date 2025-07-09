use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApplicationInformation {
    pub name: String,
    pub exec_path: String,
    pub cmd_args: Option<String>,
    pub working_dir: Option<PathBuf>,
}
