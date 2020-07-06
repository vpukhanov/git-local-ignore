use std::path::PathBuf;

use crate::git;

pub fn clear_exclude_list(repo: &git::GitRepo, force: bool) {
    if !force
        && !dialoguer::Confirm::new()
            .with_prompt("Reset the local exclude list?")
            .interact()
            .unwrap_or(false)
    {
        return;
    }

    if repo.clear_exclude_list().is_ok() {
        println!("Successfully reset the local exclude file");
    } else {
        report_error("Unable to reset the local exclude file");
    }
}

pub fn print_exclude_list(repo: &git::GitRepo) {
    let entries = repo.exclude_list().unwrap_or_else(|_err| {
        report_error("Could not load entries from the exclude file");
    });

    println!("\nEntries in the exclude file:");
    entries.for_each(|entry| println!("  {}", entry));
}

pub fn add_entries_to_exclude_list(
    repo: &git::GitRepo,
    base_path: &PathBuf,
    entries: &Vec<String>,
    force: bool,
) {
    let entries_count = entries.len();

    if !force && entries_count > 1 {
        println!("Inserting {} entries into the exclude file.", entries_count);
        println!("Hint: if you want to insert glob patterns with wildcard characters (*, ?, ...) into the exclude file as is, escape them with backslash '\\'.");

        if !dialoguer::Confirm::new()
            .with_prompt("Continue?")
            .interact()
            .unwrap_or(false)
        {
            return;
        }
    }

    if repo.append_to_exclude_list(base_path, entries).is_ok() {
        println!("Successfully inserted entries into the exclude file:");
        for entry in entries {
            println!("  {}", entry);
        }
    } else {
        report_error("Writing entries into the exclude file failed");
    }
}

pub fn report_error(description: &str) -> ! {
    eprintln!("❌ {}", description);
    std::process::exit(1);
}
