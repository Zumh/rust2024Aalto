pub fn move_file() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: mv SOURCE DEST");
        std::process::exit(1);
    }
    let source = &args[1];
    let dest = &args[2];
    mv(source, dest)?;
    Ok(())
}

fn mv(source: &str, dest: &str) -> std::io::Result<()> {
    
    std::fs::rename(source, dest)?;
    Ok(())
}

/*
// model solution using read, write, and remove file
fn mv(source: &str, dest: &str) -> std::io::Result<()> {
    let contents = std::fs::read_to_string(source)?;
    std::fs::write(dest, contents).expect("Could not write to file {dest}");
    std::fs::remove_file(source).expect("Could not remove file {source}");
    Ok(())
}
*/