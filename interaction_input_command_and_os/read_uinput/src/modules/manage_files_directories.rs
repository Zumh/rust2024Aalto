use std::{io, fs};

pub fn read_file() {
    let bytes = fs::read("./src/main.rs").expect("Unable to read file");
    println!("file contents as bytes:");
    println!("{bytes:?}");
    let string = String::from_utf8(bytes).expect("Invalid UTF-8");
    println!("file contents as string:");
    println!("{string}");
}

pub fn read_to_file_string() {
        let data = fs::read_to_string("./src/main.rs").expect("Unable to read file to UTF-8 String");
        println!("{data}");
}

pub fn write_to_file() {
    let data = "This will be deleted anyway";
    fs::write("/tmp/testing", data).expect("Unable to write file");

    // read and print the file contents to see the changes
    let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    println!("{new_contents}");
}

pub fn write_content_to_file() {
    let random_bytes = vec![255, 23, 42, 0, 222];
    fs::write("/tmp/testing", random_bytes).expect("Unable to write file");

    // read and print the file contents to see the changes
    let new_contents = fs::read("/tmp/testing").expect("Unable to read file");
    let new_contents = String::from_utf8_lossy(&new_contents);
    println!("{new_contents}");
}

pub fn write_avoid_over_writing(){
    use std::path::Path;

    let data = "This will be deleted anyway";
    if !Path::new("/tmp/testing").exists() {
        fs::write("/tmp/testing", data).expect("Unable to write file");
    }

    // read and print the file contents to see the changes
    //let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    let new_contents = fs::read("/tmp/testing").expect("Unable to read file");
    let new_contents = String::from_utf8_lossy(&new_contents);
    println!("{new_contents}");

}

pub fn appending_file() {
    use std::io::Write;

    // create an empty file to write to
    fs::write("/tmp/testing", "").expect("Unable to write file");

    // open the file in append mode
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("/tmp/testing")
        .expect("Unable to open file");

    // write "data" to the end of the file
    let data = "This will be deleted anyway";
    writeln!(file, "{}", data).expect("Unable to write data");

    // write "data" to the end of the file again
    writeln!(file, "{}", data).expect("Unable to write data");

    // read and print the file contents to see the changes
    let new_contents = fs::read_to_string("/tmp/testing").expect("Unable to read file");
    println!("{new_contents}");
}

pub fn removing_file(){
        let data = "This will be deleted anyway";
        fs::write("/tmp/testing", data).expect("Unable to write file");
        fs::remove_file("/tmp/testing").expect("Unable to remove file");
}

pub fn reading_file_in_compile_time() {
    // We can also read files at compile time with the include_str! macro. The include_str! macro will read the file at compile time and include the contents of the file as a string. 
    let cargo_toml = include_str!("../../Cargo.toml");
    fs::remove_file("./Cargo.toml").expect("Unable to remove Cargo.toml");
    println!("Project toml: {cargo_toml}");
}


pub fn listing_directories() {
    if let Err(e) = backup("main.rs") {
        println!("Error: {e}");
    }
}

fn backup(filename: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(filename)?;
    let backup = format!("// this is a backup\n{contents}");

    let backup_path = format!("{filename}.backup");
    fs::write(&backup_path, backup)?;
    println!("Contents of {filename} successfully backed up to {backup_path}!");
    Ok(())
}


// return option or result

fn read_int() -> Option<i32> {
    Some(io::stdin().lines().next()?.ok()?.parse().ok()?)
}

pub fn option_or_result () {
    let a = read_int().unwrap();
    let b = read_int().unwrap();
    println!("{a} + {b} = {}", a + b);
}

// we can also propagate errors from the main
pub fn propagate_main_error() -> Result<(), io::Error> {
    backup("lib.rs")?;
    Ok(())
}

// remove everything
pub fn create_remove() -> std::io::Result<()> {
    // return insuffecient permission
    println!("{:?}", fs::create_dir("new_dir/subdir"));
    println!("{:?}", fs::create_dir_all("new_dir/subdir"));
    println!("{:?}", fs::remove_dir("new_dir"));
    println!("{:?}", fs::remove_dir_all("new_dir"));
    Ok(())
}

pub fn os_string(){
    // The file_name method of DirEntry doesn't return a String or a &str 
    // a compatibility feature in Rust which can store data in the different encodings different operating systems use 
    //  OsString may contain non-valid UTF-8 unlike a String.
    //  An OsString can't be displayed without debug format (:?)
    // padding doesn't work on debug format, so we need to get a String or &str from the OsString.
    // an OsString to a &str is to use the to_string_lossy method
    // where invalid unicode characters are replaced with �
    // technially return Clone-on-write smart pointer Cow<str>,
    let os_string = std::ffi::OsString::from("chicken");
    let string = os_string.to_string_lossy();
    println!("{string:>20}"); // chicken!
}

pub fn temp_drop_before_use() -> std::io::Result<()> {

    use std::fs::read_dir;
    for entry in read_dir(".")? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        
        // entry.file_name() returns a new OsString, which is not a reference to entry. 
        // the referenced OsString gets dropped
        let filename = entry.file_name();
        // assigned to a new variable expand the life time of the string
        let filename = filename.to_string_lossy();
        println!(
            "{filename:>20}: is_dir = {:<5}, len = {:<8}, created = {:?}",
            metadata.is_dir(),
            metadata.len(),
            metadata.created()
        );
    }
    Ok(())
}

