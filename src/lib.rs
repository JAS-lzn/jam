mod fetchers;
mod parsers;
mod tests;

use std::fs;

use fetchers::*;
use git2::Repository;
use parsers::*;

// public API exposed to consumers of `jam`
// TODO: Make the input args real
pub fn jam_finder(arg: String) {
    // TODO: give args to let people choose to just grab diffs,
    git_data(FetchOptions {
        repo_url: Some(arg),
        diff: None,
    });
    // TODO: getDiffs(versionA, versionB)
    find_methods("uwu".to_string());
}

// public API to obtain a list of diffs for the main branch of a repo
pub fn diff_finder(arg_path: String) -> Vec<String> {
    // check for a repo in memory
    let has_working_dir = fs::metadata(&arg_path).is_ok();

    // if we don't have a repo cloned, warn the user and exit
    if has_working_dir == false {
        print!("There is no repo in memory! Clone a repo first")
    }

    let files_in_repo = find_methods(arg_path);

    return files_in_repo;

    // if we *DO* have a repo in memory, continue
}
