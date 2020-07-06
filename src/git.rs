use std::path::{Path, PathBuf};

pub struct GitRepo {
    pub dir: PathBuf,
}

pub fn find_repo(target_dir: &PathBuf) -> Option<GitRepo> {
    let mut current_dir: Option<&Path> = Some(target_dir.as_path());

    while let Some(dir) = current_dir {
        let potential_repo = dir.join(".git");

        if potential_repo.exists() {
            return Some(GitRepo {
                dir: potential_repo,
            });
        }

        current_dir = dir.parent();
    }

    None
}
