
/*
Write a function append_to_file that takes in two filenames as arguments and appends the contents of the first file to the second file. The function should create the target file if one does not exist.

The function should return a Result type, where the Ok variant contains the unit type () and the Err variant contains an std::io::Error.

The exercise template contains a main function that can be used to test the append_to_file function.
*/
pub fn append_to_file_main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: append SOURCE DEST");
        std::process::exit(1);
    }
    let source = &args[1];
    let dest = &args[2];
    append_to_file(source, dest)?;
    Ok(())
}

// create if not exists other wise append
fn append_to_file(source: &str, dest: &str) -> std::io::Result<()> { 
    let mut reader = std::fs::File::open(source)?;
    let mut writer = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(dest)?;
    std::io::copy(&mut reader, &mut writer)?;
    Ok(())
}

// model solution 
/*

use std::io::Write;

fn append_to_file(source: &str, dest: &str) -> std::io::Result<()> {
    let to_append = std::fs::read_to_string(source)?;
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(dest)?;
    write!(file, "{}", to_append).expect("Unable to write data");
    Ok(())
}
*/