fn main() {
 let mut text = String:from("Once");
 let editable_borrow = &mut text;
  editable_borrow.push_str(" upon");
  editable_borrow.push_str(" a time...");
  let story = &text;
  println!("{story}");
}
