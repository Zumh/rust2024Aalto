use std::fs;

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
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => return Err(e),
    };
    let backup = format!("// this is a backup\n{contents}");

    let backup_path = format!("{filename}.backup");
    match fs::write(&backup_path, backup) {
        Ok(_) => println!("Contents of {filename} successfully backed up to {backup_path}!"),
        Err(e) => return Err(e),
    };
    Ok(())
}



