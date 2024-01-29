## Running rust on computer

## Rustup
- rustc
  - https://rustup.rs/
  - Rust compiler
  - Cargo cargo > package manager and build tool
  - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  - rustup self uninstall
 
### Cargo
- https://doc.rust-lang.org/cargo/index.html
- Create rust project and run rust from command line
  - https://doc.rust-lang.org/cargo/getting-started/first-steps.html
- create project
  - cargo new hello_world
  - default --bin for binary program
    - cargo new --bin hello_world or cargo new --lib hello_world // for library
  - cd hello_world
  - Cargo.toml is a metadata that Cargo needs for compiling the package
  - manifest
- cargo build
 - compiling a rust program
 - 
- running program
  - ./target/debug/hello_world 
  - cargo run hello_world
  - cargo run

## IDE for Rust
- VSCODE with rust analyzer
  - https://code.visualstudio.com/docs/languages/rust
- rust analyzer
  - https://rust-analyzer.github.io/
  - https://rust-analyzer.github.io/manual.html

## REPL
https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md
  - Read-Evaluate-Print-Loop
    - user interactive code input executed and print out
  - Made by google
  - installation
    -  $ rustup component add rust-src
    -  $ cargo install evcxr_repl
    -  start program evcxr
    -  alias rust-repl=evcxr
We can also do it in shell instance
## Shell instance
$ . $HOME/.bashrc
$ rust-repl
$ rust-repl

Welcome to evcxr. For help, type :help
