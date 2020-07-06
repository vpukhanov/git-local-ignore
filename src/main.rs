use clap::{App, Arg};
use std::env;

mod cli;
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
        cli::report_error("Unable to access current working dir");
    });

    let git_repo = git::find_repo(&working_dir).unwrap_or_else(|| {
        cli::report_error(
            "Unable to find git repository in current directory or any of the parent directories",
        );
    });

    println!(
        "Found .git repository in {}",
        git_repo.repo_dir.to_str().unwrap()
    );

    let force = matches.is_present("force");

    if matches.is_present("clear") {
        cli::clear_exclude_list(&git_repo, force);
    } else if matches.is_present("list") {
        cli::print_exclude_list(&git_repo);
    } else {
        let files = matches.values_of_lossy("file").unwrap_or_else(|| {
            cli::report_error("No exclude entries provided");
        });

        cli::add_entries_to_exclude_list(&git_repo, &working_dir, &files, force);
    }
}
