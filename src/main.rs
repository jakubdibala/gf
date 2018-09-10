extern crate colored;
extern crate git2;

use std::path::Path;
use colored::*;

mod repository;


const REPO_PATH: &str = "/Users/jd/code/gitflow-example";


/**
 * Main function 
 */
fn main() {


    // check git repo
    if !check_git_is_initialized() {
        log_error("Git repository is not initialized here!");
        return;
    }

    // open repository
    let repo = match git2::Repository::open(Path::new(REPO_PATH)) {
        Ok(r)       => r,
        Err(e)      => {
            log_error(format!("Failed to open repository!\nDetailed information: {}", e).as_str());
            return;
        }
    };

    let current = repository::current_branch(repo);
}


/**
 * Check if git repository is
 * initialized in current directory
 */
fn check_git_is_initialized() -> bool {
    let mut p: String = String::from(REPO_PATH);
    p.push_str("/.git");
    Path::new(&p).exists()
}


/**
 * Print out error message
 */
fn log_error(message: &str) {
    println!("{} {}", "[ERROR]".red().bold(), message);
}
