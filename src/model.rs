use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ApplicationInformation {
    pub name: String,
    pub exec_path: String,
    pub cmd_args: Option<String>,
    pub working_dir: Option<PathBuf>,
}
