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
                .short('l')
                .long("list")
                .about("List currently excluded files"),
        )
        .arg(
            Arg::with_name("file")
                .index(1)
                .multiple_values(true)
                .about("Files to exclude from index"),
        )
        .get_matches();

    let working_dir = env::current_dir().unwrap_or_else(|_err| {
        report_error("Unable to access current working dir", 1);
    });

    let git_repo = git::find_repo(&working_dir).unwrap_or_else(|| {
        report_error(
            "Unable to find git repository in current directory or any of the parent directories",
            1,
        );
    });

    println!(
        "Found .git repository in {}",
        git_repo.dir.to_str().unwrap()
    );

    if matches.is_present("list") {
        println!("List mode");
    } else {
        println!("Add mode");
    }
}

fn report_error(description: &str, exit_code: i32) -> ! {
    eprintln!("❌ {}", description);
    std::process::exit(exit_code);
}
