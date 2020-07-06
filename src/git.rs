use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

pub struct GitRepo {
    pub dir: PathBuf,
}

impl GitRepo {
    pub fn exclude_list(&self) -> Result<impl Iterator<Item = String>, io::Error> {
        use io::BufRead;

        let exclude_file = File::open(self.exclude_file_path()?)?;
        Ok(io::BufReader::new(exclude_file)
            .lines()
            .filter_map(Result::ok)
            .filter(|line| !line.starts_with("#")))
    }

    pub fn append_to_exclude_list(&self, entries: &Vec<String>) -> Result<(), io::Error> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut exclude_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.exclude_file_path()?)?;

        for entry in entries {
            writeln!(exclude_file, "{}", entry);
        }

        Ok(())
    }

    fn exclude_file_path(&self) -> Result<PathBuf, io::Error> {
        let exclude_file_path = self.dir.join("info/exclude");

        if !exclude_file_path.exists() {
            File::create(&exclude_file_path)?;
        }

        Ok(exclude_file_path)
    }
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
