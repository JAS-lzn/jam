extern crate git2;

use git2::Repository;

pub fn git_data(options: FetchOptions) {
    if options.repo_url.is_some() {
        repo_cloner(&options.repo_url.unwrap())
    }
    if options.diff.is_some() {
        diff_data(options.diff.unwrap());
    }
}

fn repo_cloner(url: &str) {
    println!("Cloning repo: {}", url);
    let git_name = url.split("/").collect::<Vec<&str>>().pop().unwrap();
    let repo_name = git_name.replace(".git", "");
    match Repository::clone(url, format!("./working_dir/{}", repo_name)) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

fn diff_data(diff: DiffOptions) -> String {
    // Do some stuff to find the diff between our two given versions
    diff.branch_a.to_owned() + &diff.branch_b
}

// I made these optional since a user could either want
// to clone a whole repo, or maybe just examine one specific diff
// This is actually terrible, but I'll come back later
pub struct FetchOptions {
    pub repo_url: Option<String>,
    pub diff: Option<DiffOptions>,
}

pub struct DiffOptions {
    pub branch_a: String,
    pub branch_b: String,
}
