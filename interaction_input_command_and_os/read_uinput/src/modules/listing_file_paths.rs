

/*
Write a function list_dir_entry_paths that takes in a path as argument and prints out the paths to all files and directories in the given path. If the path is a directory, the function should print out the paths ending with a slash (/) to indicate that it is a directory.

As an example, if the function is called with the path "src" as the argument and the directory src contains the files main.rs and lib.rs and a directory util, the function should print out
src/main.rs
src/lib.rs
src/util/
The exercise template contains a main function that can be used to test the list_dir_entry_paths function.

*/
pub fn listing_file_paths() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: find PATH");
        std::process::exit(1);
    }
    let path = &args[1];
    list_dir_entry_paths(path)?;
    Ok(())
}

// Hint: Check out the available methods for std::fs::DirEntry and std::path::PathBuf. The to_string_lossy method should prove especially helpful here.


fn list_dir_entry_paths(path: &str) -> std::io::Result<()> {
    let dir = std::fs::read_dir(path)?;
    for entry in dir {
        println!("{}", entry?.path().to_string_lossy());
    }
    Ok(())
}

// model solution
/*
use std::env;
use std::fs;
use std::io;
fn list_dir_entry_paths(path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            println!("{}/", path.to_string_lossy());
        } else {
            println!("{}", path.to_string_lossy());
        }
    }
    Ok(())
}
*/




