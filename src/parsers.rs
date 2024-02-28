use std::fs;

// TODO: identify functions in method text
pub fn find_methods(repo_directory: String) -> Vec<String> {
    return navigate_files(repo_directory);
}

// TODO: recursively navigate thru files
fn navigate_files(arg_directory: String) -> Vec<String> {
    // Identify all the files in the current directory
    // stringify all those files
    // Invoke the 'find_methods' method for each of the files
    // repeat recursively for each directory in the current directory
    let paths = fs::read_dir(arg_directory).unwrap();

    let mut path_holder = Vec::new();
    for path in paths {
        path_holder.push(path.unwrap().path().display().to_string());
    }
    return path_holder;
}

// TODO: Identify methods in files
/*
fn find_methods(){
    // make gross reg exp that idenfities method names, arguments, and return types
    // return a list of funtion names, arguments, and return types
}
*/
