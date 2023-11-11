mod fetchers;
mod parsers;

use fetchers::*;
use parsers::*;

// public API exposed to consumers of `jam`
// TODO: Make the input args real
pub fn jam_finder(arg: String) {
	// TODO: give args to let people choose to just grab diffs,
	git_data(FetchOptions{repo_url: Some(arg), diff: None});
	// TODO: getDiffs(versionA, versionB)
	find_methods("uwu".to_string());
}
