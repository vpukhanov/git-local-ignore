use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use clap_generate::generators::*;
use clap_generate::{generate, Generator};
use std::env;
use std::io;

mod cli;
mod git;

// cli interface
fn cli_gl() -> App<'static> {
    App::new("git-local-ignore")
        .version(crate_version!())
        .author(crate_authors!())
        .about(
            "Locally exclude files from being tracked by Git (without adding them to .gitignore)",
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .about("Ignore any additional prompts, assumes 'yes' as the answer"),
        )
        .arg(
            Arg::new("clear")
                .conflicts_with_all(&["list", "file"])
                .short('c')
                .long("clear")
                .about("Clear all entries from the exclude file"),
        )
        .arg(
            Arg::new("list")
                .conflicts_with("file")
                .short('l')
                .long("list")
                .about("List all entries in the exclude file"),
        )
        .arg(
            Arg::new("file")
                //.required_unless_present_any(&["list", "clear"])
                .short('f')
                .long("file")
                .multiple(true)
                //.index(1)
                .about("Entries to add to the exclude file"),
        )
        .subcommand(
            App::new("completion")
                .version(crate_version!())
                .author(crate_authors!())
                .setting(AppSettings::Hidden)
                .about("AutoCompletion")
                .arg(
                    Arg::new("shell")
                        .short('s')
                        .long("shell")
                        .about("Selects shell")
                        .required(true)
                        .takes_value(true)
                        .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"]),
                )
                .arg(
                    Arg::new("manual")
                        .short('m')
                        .long("manual")
                        .about("Display instructions on how to install autocompletions"),
                ),
        )
}

/// Print completions
fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let matches = cli_gl().get_matches();

    let working_dir = env::current_dir().unwrap_or_else(|_err| {
        cli::report_error("Unable to access current working dir");
    });

    let git_repo = git::find_repo(&working_dir).unwrap_or_else(|| {
        cli::report_error(
            "Unable to find git repository in current directory or any of the parent directories",
        );
    });

    //println!(
    //"Found .git repository in {}",
    //git_repo.repo_dir.to_str().unwrap()
    //);

    let force = matches.is_present("force");

    if matches.is_present("clear") {
        cli::clear_exclude_list(&git_repo, force);
    } else if matches.is_present("list") {
        cli::print_exclude_list(&git_repo);
    } else if matches.is_present("file") {
        let files = matches.values_of_lossy("file").unwrap_or_else(|| {
            cli::report_error("No entries to exclude provided");
        });

        cli::add_entries_to_exclude_list(&git_repo, &working_dir, &files, force);
    }

    if let Some(matches) = matches.subcommand_matches("completion") {
        let shell = matches.value_of("shell").unwrap();

        let mut app = cli_gl();
        match shell {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => panic!("Unknown generator"),
        }

        //if matches.is_present("manual") {
        // TODO: manual
        //}
    }
}
