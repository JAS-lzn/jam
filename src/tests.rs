use std::ptr::null;

#[test]
fn can_read_files() {
 let files = crate::diff_finder("m".to_owned());
 assert!(files.len() > 1);

 println!("{}", files.join(" "))
}

#[test]
fn can_get_repo() {
    crate::jam_finder("https://github.com/jmlopez-rod/m".to_string());

    // cleanup
    let _ = std::fs::remove_dir_all("/working_dir/m");
}

