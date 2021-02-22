use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use clap_generate::generators::*;
use clap_generate::{generate, Generator};
use colored::*;
use std::env;
use std::io;
use std::process::exit;

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
                .short('t')
                .long("target")
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
        let manual = matches.is_present("manual");

        let shell = matches.value_of("shell").unwrap();

        if manual {
            match shell {
              "bash" => print!(
                "\n{sh}\n{cmd}\n\n{cmt}\n{lx}\n{lxcmd}\n\n{os}\n{oscmd}\n",
                sh = "Bash:".blue().bold(),
                cmd = "$ source <(chapulin AC --shell bash)".cyan(),
                cmt = "# To load completions for each session, execute once:".yellow().dimmed(),
                lx = "Linux:".green(),
                lxcmd = "$ chapulin AC --shell bash > /etc/bash_completion.d/chapulin".cyan(),
                os = "MacOS:".green(),
                oscmd = "$ chapulin AC --shell bash > /usr/local/etc/bash_completion.d/chapulin".cyan(),
                ),

              "elvish" => print!("\n{}\n{}\n", "Documentation not available. Apologies.".green(), "Come back soon...".cyan(), ),

              "fish" => print!(
                "\n{sh}\n\n{cmd1}\n\n{cmt}\n{cmd2}\n",
                sh = "Fish:".blue().bold(),
                cmd1 = "$ chapulin AC --shell fish | source".cyan(),
                cmt = "# To load completions for each session, execute once:".yellow().dimmed(),
                cmd2 = "$ chapulin AC --shell fish > ~/.config/fish/completions/chapulin.fish".cyan(),
              ),

              "powershell" => print!(
                "\n{sh}\n\n{cmd1}\n\n{cmt1}\n{cmd2}\n\n{cmt2}\n",
                sh = "Powershell:".blue().bold(),
                cmd1 = "PS> chapulin AC --shell powershell | Out-String | Invoke-Expression".cyan(),
                cmt1 = "# To load completions for every new session, run:".yellow().dimmed(),
                cmd2 = "PS> chapulin AC --shell powershell > chapulin.ps1".cyan(),
                cmt2 = "# and source this file from your powershell profile".yellow().dimmed(),
              ),

              "zsh" => print!(
                "\n{sh}\n\n{cmt1}\n{cmt2}\n{cmd1}\n\n{cmt3}\n{cmd2}\n\n{cmt4}\n",
                sh = "Zsh:".blue().bold(),
                cmt1 = "# If shell completion is not already enabled in your environment you will need".yellow().dimmed(),
                cmt2 = "# to enable it.  You can execute the following once:".yellow().dimmed(),
                cmd1 = "$ echo \"autoload -U compinit; compinit\" >> ~/.zshrc".cyan(),
                cmt3 = "# To load completions for each session, execute once:".yellow().dimmed(),
                cmd2 = "$ chapulin AC --shell zsh > \"${fpath[1]}/_chapulin\"".cyan(),
                cmt4 = "# You will need to start a new shell for this setup to take effect".yellow().dimmed(),
              ),

              _ => panic!("Unknown generator"),
            }

            exit(0);
        }

        let mut app = cli_gl();
        match shell {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => panic!("Unknown generator"),
        }
    }
}
