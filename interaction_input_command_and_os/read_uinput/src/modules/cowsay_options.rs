const COWSAY: &str = r#"
\  ^__^
 \ (oo)\_______
   (__)\       )\/\
       ||----w |
       ||     ||
"#;

const DRAGONSAY: &str = r#"
\
 \                       / \  //\
  \        |\___/|      /   \//  \\
   \       /0  0  \__  /    //  | \ \
    \     /     /  \/_/    //   |  \  \
          @_^_@'/   \/_   //    |   \   \
          //_^_/     \/_ //     |    \    \
       ( //) |        \///      |     \     \
     ( / /) _|_ /   )  //       |      \     _\
   ( // /) '/,_ _ _/  ( ; -.    |    _ _\.-~        .-~~~^-.
 (( / / )) ,-{        _      `-.|.-~-.           .~         `.
(( // / ))  '/\      /                 ~-. _ .-~      .-~^-.  \
(( /// ))      `.   {            }                   /      \  \
 (( / ))     .----~-.\        \-'                 .~         \  `. \^-.
            ///.----..>        \             _ -~             `.  ^-`  ^-_
              ///-._ _ _ _ _ _ _}^ - - - - ~                     ~-- ,.-~
                                                                 /.-~
"#;

const TUXSAY: &str = r#"
\   .--.
 \ |o_o |
   |:_/ |
  //   \ \
 (|     | )
/'\_   _/`\
\___)=(___/
"#; 

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}
// create a hash that match dragon, tux and cow


pub fn cow_say_options() {
    let args: Vec<String> = std::env::args().collect();
    let mut animal: &str = COWSAY;
    let mut message = "Moo!".to_string();
    // model solution 
    /*
     let message = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "Moo!".to_string());
     */

    println!("args: {}", args.len());
    // If the animal is not supported, the program should print an error message Unsupported animal: {arg} and exit with a non-zero exit code (panic).
    match args.len() {
        1..=2 => {
          animal = COWSAY;
          message = args.get(1).cloned().unwrap_or_else(|| "Moo!".to_string());
        },
        3 => {
          
          match args[1].trim().to_lowercase().as_str() {

            "cow" => animal = COWSAY,
            "dragon" => animal = DRAGONSAY,
            "tux" => animal = TUXSAY,
            _ => {
              eprintln!("Unsupported animal: {}", args[1]);
              std::process::exit(1);
            }
          }

          message = args.get(2).cloned().unwrap_or_else(|| "Moo!".to_string());
              
        },
        _ => {
            eprintln!("Invalid number of arguments.");
            std::process::exit(1);
        }
    }


  

    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);
    let result = animal  
        .trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{result}");
}
