# Fancy Commit

## Installation


<script id="asciicast-0Ttau0hppgC9UQlTWN0OWeLM9" src="https://asciinema.org/a/0Ttau0hppgC9UQlTWN0OWeLM9.js" async></script>



### Archlinux

This package can be easily built using ArchLinux, just go in release panel and download archpkg.tar.gz
Extract it, then just run `makepkg -sri` and it will be installed !

I will try to put package on AUR repositories :D

### Other distributions

You will have to install rust toolchain before being able to build fancy_commit. If you have `cargo`, you can continue !

Just clone this repository, run `cargo build --release && sudo cp target/release/fancy_commit /usr/bin/`
And you'll be done !

## Usage

Firstly, you'll have to tell git to use fancy_commit ! 

In order to do that, just run `git config core.editor "fancy_commit vim"` (replace vim by your favorite editor)

Then, fancy commit will run each time you commit, but if git tries to open editor to do anything else than a commit, the second editor you choose will be executed !
