extern crate colored;
extern crate git2;
extern crate clap;

use std::path::Path;
use colored::*;
use clap::{Arg, App, SubCommand};

mod repository;

#[derive(Debug)]
enum Verbosity {
    Quiet,
    Verbose
}


const REPO_PATH: &str = "/Users/jd/code/gitflow-example";


/**
 * Main function 
 */
fn main() {
    // open repository
    let repo = match git2::Repository::open(Path::new(REPO_PATH)) {
        Ok(r)       => r,
        Err(e)      => {
            log_error(format!("Failed to open repository!\nDetailed information: {}", e).as_str());
            return;
        }
    };

    // configure app args
    let matches = App::new("Git flow tool")
                    .version("1.0")
                    .author("Jakub Dibala <jakub.dibala@brainz.cz>")
                    .about("This tool is an easier way to use git flow (the right way)")
                    .arg(
                        Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Enables verbose output")
                    )
                    .arg(
                        Arg::with_name("target")
                            .help("feature/hotfix/release name")
                    )
                    .get_matches();

    // get verbose level
    let verbosity = match matches.occurrences_of("verbose") {
        0 => Verbosity::Quiet,
        _ => Verbosity::Verbose,
    };

    println!("{:?}", verbosity);
}



/**
 * Print out error message
 */
fn log_error(message: &str) {
    println!("{} {}", "[ERROR]".red().bold(), message);
}
