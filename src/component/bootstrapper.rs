use std::error::Error;
use std::fs;
use std::path::Path;

use git2::Repository;

pub struct Bootstrapper {
    foobar: u32
}

impl Bootstrapper {
    pub fn new() -> Self {
        Bootstrapper { foobar : 42 }
    }

    pub fn get_repo(&self, repo_path: &Path) -> Result<Repository, Box<dyn Error>> {
        if !repo_path.exists() {
            fs::create_dir_all(repo_path)?;
        } else if !repo_path.is_dir() {
            panic!("Not a directory")
        }

        match Repository::open(repo_path) {
            Ok(repo) => Ok(repo),
            Err(_) => match Repository::init(repo_path) {
                Ok(repo) => Ok(repo),
                Err(_) => panic!("Cannot open or create")
            }
        }
    }
}
