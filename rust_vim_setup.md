## install vim plug
https://github.com/junegunn/vim-plug
```bsh
curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
```
## install latest vim
To install the latest stable version of Vim on Ubuntu, you can use a PPA (Personal Package Archive) to get the most recent version. Here are the steps:

1. First, ensure that your package lists are up to date by running:
   ```bash
   sudo apt update
   ```

2. Install the software-properties-common package if you don't already have it, which allows you to easily manage your repositories:
   ```bash
   sudo apt install software-properties-common
   ```

3. Next, add the PPA maintained by Jonathon F:
   ```bash
   sudo add-apt-repository ppa:jonathonf/vim
   ```

4. Now, update your package lists again:
   ```bash
   sudo apt update
   ```

5. Finally, install Vim:
   ```bash
   sudo apt install vim
   ```

This will install the latest stable version of Vim available in the PPA repository.

After the installation is complete, you can verify the version of Vim installed by running:
```bash
vim --version
```

This should display information about the installed version of Vim, confirming that you have the latest stable version installed.
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
