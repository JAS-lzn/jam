use std::ptr::null;

#[test]
fn can_get_repo() {
    crate::jam_finder("https://github.com/jmlopez-rod/m".to_string());

    // cleanup
    let _ = std::fs::remove_dir_all("/working_dir/m");
}

#[test]
fn can_read_files() {
    let files = crate::diff_finder("working_dir/m".to_owned());
    println!("{}", files.join(" "));

    assert!(files.len() > 1);
}
