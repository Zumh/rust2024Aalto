pub fn print_if_content(maybe_content: Option<&str>) {
    if let Some(content) = maybe_content {
        println!("{content}");
    }
}

pub fn panic_if_no_content(maybe_content: Option<&str>) {
    if maybe_content.is_none() {
        panic!("No content!");
    }
}

pub fn check_content(maybe_content: Option<&str>) {
    match maybe_content {
        Some(content) => println!("Has content: {content}"),
        None => println!("No content!"),
    }
}

fn main() {
    print_if_content(Some("✅")); // Should print "✅"
    print_if_content(None); // Should not print anything

    check_content(Some("✅")); // Should print "Has content: ✅"
    check_content(None); // Should print "No content!"

    panic_if_no_content(Some("✅")); // Should not panic
    panic_if_no_content(None); // Should panic with message "No content!"
}
