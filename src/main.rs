use clap::{App, Arg};
use std::env;

mod git;

fn main() {
    let matches = App::new("git local-ignore")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vyacheslav P. <vyacheslav.pukhanov@gmail.com>")
        .about("Locally exclude files from the Git index")
        .arg(
            Arg::with_name("force")
                .short('f')
                .long("force")
                .about("Ignore any additional prompts, assume 'yes' as an answer"),
        )
        .arg(
            Arg::with_name("clear")
                .conflicts_with_all(&["list", "file"])
                .short('c')
                .long("clear")
                .about("Remove all entries from the exclude file"),
        )
        .arg(
            Arg::with_name("list")
                .conflicts_with("file")
                .short('l')
                .long("list")
                .about("List currently excluded files"),
        )
        .arg(
            Arg::with_name("file")
                .required_unless_one(&["list", "clear"])
                .index(1)
                .multiple(true)
                .about("Files to exclude from index"),
        )
        .get_matches();

    let working_dir = env::current_dir().unwrap_or_else(|_err| {
        report_error("Unable to access current working dir");
    });

    let git_repo = git::find_repo(&working_dir).unwrap_or_else(|| {
        report_error(
            "Unable to find git repository in current directory or any of the parent directories",
        );
    });

    println!(
        "Found .git repository in {}",
        git_repo.dir.to_str().unwrap()
    );

    let force = matches.is_present("force");

    if matches.is_present("clear") {
        clear_exclude_list(&git_repo, force);
    } else if matches.is_present("list") {
        print_exclude_list(&git_repo);
    } else {
        let files = matches.values_of_lossy("file").unwrap_or_else(|| {
            report_error("No exclude entries provided");
        });

        add_entries_to_exclude_list(&git_repo, &files, force);
    }
}

fn clear_exclude_list(repo: &git::GitRepo, force: bool) {
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

fn print_exclude_list(repo: &git::GitRepo) {
    let entries = repo.exclude_list().unwrap_or_else(|_err| {
        report_error("Could not load entries of the exclude file");
    });

    println!("\nEntries of the exclude file:");
    entries.for_each(|entry| println!("  {}", entry));
}

fn add_entries_to_exclude_list(repo: &git::GitRepo, entries: &Vec<String>, force: bool) {
    let entries_count = entries.len();

    if !force && entries_count > 1 {
        println!("Inserting {} entries into the exclude file.", entries_count);
        println!("Hint: if you want to insert wildcard characters (*, ?, ...) into the exclude file as is, escape them with backslash '\\'.");

        if !dialoguer::Confirm::new()
            .with_prompt("Continue?")
            .interact()
            .unwrap_or(false)
        {
            return;
        }
    }

    if repo.append_to_exclude_list(entries).is_ok() {
        println!("Successfully inserted entries into the exclude file:");
        for entry in entries {
            println!("  {}", entry);
        }
    } else {
        report_error("Writing entries into the exclude file failed");
    }
}

fn report_error(description: &str) -> ! {
    eprintln!("❌ {}", description);
    std::process::exit(1);
}
