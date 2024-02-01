fn add_to_story(story: &mut String, text: &str) {
  story.push_str(text);
}

fn main() {

  let mut story = String::new();
  add_to_story(&mut story, "Once upon a time, ");
  add_to_story(&mut story, "there was a little mouse.");
  println!("{story}");
}
