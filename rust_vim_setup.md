## Vim 
```vimplugin
 call plug#begin()
 Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
 Plug 'junegunn/fzf.vim'
 Plug 'rust-lang/rust.vim'
 Plug 'Exafunction/codeium.vim', { 'branch': 'main' }
 call plug#end()
 set relativenumber
```
## Codeium setup
- :Codeium Auth
- token paste from the website

## vim plug.in 
- PlugInstall
- PlugUpdate
- so % (for refresh)

## fzf

- :Files -> for searching files in fzf
