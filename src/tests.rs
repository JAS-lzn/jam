use std::{fs::{self, remove_dir_all}, io, path::Path};


fn file_remover_helper(arg_path: &Path) -> Result<bool, io::Error> {
	if arg_path.exists(){
		remove_dir_all(arg_path)?;
	}
	return Ok(true)
}

#[test]
fn can_get_repo() {
	let removal = file_remover_helper(Path::new("working_dir/m"));

	assert_eq!(removal.is_ok(), true, "check if the working directory already exists");

	let _dir_removal_msg = fs::remove_dir_all("/working_dir/m");
	
    crate::jam_finder(&"https://github.com/jmlopez-rod/m".to_string());
}

#[test]
fn can_read_files() {
    let files = crate::method_finder(&"working_dir/m".to_string());
    println!("{}", files.join(" "));

    assert!(files.len() > 1);
}


