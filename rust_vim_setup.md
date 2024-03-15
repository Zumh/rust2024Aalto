## Rust Vim 
- https://github.com/rust-lang/rust.vim
```vimplugin
 call plug#begin()
 Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
 Plug 'junegunn/fzf.vim'
 Plug 'rust-lang/rust.vim'
 Plug 'Exafunction/codeium.vim', { 'branch': 'main' }
 call plug#end()
 set relativenumber
```
## Codeium setup Vim
- https://codeium.com/vim_tutorial
- :Codeium Auth
- token paste from the website

## vim plug.in 
- PlugInstall
- PlugUpdate
- so % (for refresh)

## fzf

- :Files -> for searching files in fzf
## compile rust normally 
- rustc hello.rs
## how to run rust and debug 
- rustc -g hello.rs
- ./hello
- gdb ./hello
