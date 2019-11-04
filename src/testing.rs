use std::path::Path;
use std::process::Command;

use git2;
use tempfile;

pub struct TestingGitRepo {
    repo: git2::Repository,
    tempdir: tempfile::TempDir
}

impl TestingGitRepo {
    pub fn new() -> Self {
        let tempdir = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init(tempdir.path()).unwrap();

        Self {
            repo: repo,
            tempdir: tempdir
        }
    }

    pub fn path<'a>(&'a self) -> &'a Path {
        self.tempdir.path()
    }

    pub fn status(&self) -> String {
        let repo_path = self.tempdir.path().to_str().unwrap();
        let stdout = Command::new("git")
            .args(&["status"])
            .current_dir(&repo_path)
            .output()
            .expect("failed to get git status")
            .stdout;
        std::str::from_utf8(&stdout).unwrap().to_owned()
    }
}
