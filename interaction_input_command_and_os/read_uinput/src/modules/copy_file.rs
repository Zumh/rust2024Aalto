pub fn copy_file() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cp SOURCE DEST");
        std::process::exit(1);
    }
    let source = &args[1];
    let dest = &args[2];

    copy(source, dest)?;

    Ok(())
}

fn copy(source: &str, dest: &str) -> std::io::Result<()> {
    let mut reader = std::fs::File::open(source)?;
    let mut writer = std::fs::File::create(dest)?;
    std::io::copy(&mut reader, &mut writer)?;
    Ok(())  
}

// model solution
/*
fn copy(source: &str, dest: &str) -> std::io::Result<()> {
    // assuming that source and dest are valid paths and source file exists
    let contents = std::fs::read_to_string(source)?;
    std::fs::write(dest, contents)?;
    // or use std::fs::copy(source, dest)?;
    Ok(())
}
*/


