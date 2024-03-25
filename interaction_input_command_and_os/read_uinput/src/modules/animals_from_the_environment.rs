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

const COWSAY: &str = r#"
\  ^__^
 \ (oo)\_______
   (__)\       )\/\
       ||----w |
       ||     ||
"#;

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}

pub fn animals_from_the_environment() {
    let args: Vec<String> = std::env::args().collect();
    /*// Model solution
    
    let envs = std::env::vars().collect::<HashMap<String, String>>();
    // chek if args is greater than 2 then we want specific animal from command line
    // cargo run -- tux "I'lm from an environment variable"
    let animal = if args.len() > 2 {
        args[1].as_str()
    } else {
        // we are getting from environment
        // export COWSAY_ANIMAL=dragon; cargo run -- "I'm from an environment variable"
        &envs
            .get("COWSAY_ANIMAL")
            .map(|s| s.as_str())
            .unwrap_or_else(|| "cow")
    };
    
     */
    let animal = match args.len() {
        3 => {
            // chek if args is greater than 2 then we want specific animal from command line
            // cargo run -- tux "I'lm from an environment variable"
            if args.len() < 3 {
                "cow".to_string()
            } else {
                args[1].to_lowercase()
            }
        }
        _ => {
            // we are getting from environment
            // export COWSAY_ANIMAL=dragon; cargo run -- "I'm from an environment variable"
            match std::env::var("COWSAY_ANIMAL") {
                Ok(val) => {
                    match val.to_lowercase().as_str() {
                        "" => "cow".to_string(),
                        _ => val.to_lowercase(),
                    }
                }
                Err(_) => "cow".to_string(),
            }
        }
    };
    
    let message = if args.len() < 2 {
        "Moo!"
    } else if args.len() < 3 {
        args[1].as_str()
    } else {
        args[2].as_str()
    };

    let art = match animal.to_lowercase().as_str() {
        "dragon" => DRAGONSAY,
        "tux" => TUXSAY,
        "cow" => COWSAY,
        _ => panic!("Unknown animal: {message}"),
    };

    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);
    let art = art
        .trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", art);
 
}
