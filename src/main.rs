use clap::{App, Arg};
use std::env;

mod git;

fn main() {
    let matches = App::new("git local-ignore")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vyacheslav P. <vyacheslav.pukhanov@gmail.com>")
        .about("Locally exclude files from Git index")
        .arg(
            Arg::with_name("list")
                .conflicts_with("file")
                .short('l')
                .long("list")
                .about("List currently excluded files"),
        )
        .arg(
            Arg::with_name("file")
                .required_unless("list")
                .index(1)
                .multiple_values(true)
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

    if matches.is_present("list") {
        print_exclude_list(&git_repo);
    } else {
        println!("Add mode");
    }
}

fn print_exclude_list(repo: &git::GitRepo) {
    let entries = repo.exclude_list().unwrap_or_else(|_err| {
        report_error("Could not load entries of the exclude file");
    });

    println!("\nEntries of the exclude file:");
    entries.for_each(|entry| println!("  {}", entry));
}

fn report_error(description: &str) -> ! {
    eprintln!("❌ {}", description);
    std::process::exit(1);
}
