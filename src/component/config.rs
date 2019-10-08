use std::path::PathBuf;

pub struct Config {
    pub repo_path: PathBuf,
    pub hidden_repo_path: PathBuf
}

impl Config {
    pub fn new(repo_path: PathBuf, hidden_repo_path: PathBuf) -> Self {
        Self { repo_path: repo_path, hidden_repo_path: hidden_repo_path }
    }
}
