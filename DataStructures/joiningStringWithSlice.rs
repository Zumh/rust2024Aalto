fn lines_joined(lines: &[&str]) -> String {
    lines.join("\n")
}

fn main() {
    let lines = ["fn main {", r#"    println!("test");"#, "}", "", "fn unused() {", "    todo!();", "}"];
    let joined = lines_joined(&lines);
    println!("{joined}");

    let first_three_lines = &lines[..3];
    let joined = lines_joined(first_three_lines);
    println!("{joined}");
}
