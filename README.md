[![Build](https://github.com/kg6zjl/pick/actions/workflows/build.yml/badge.svg)](https://github.com/kg6zjl/pick/actions/workflows/build.yml) [![Tests](https://github.com/kg6zjl/pick/actions/workflows/test.yml/badge.svg)](https://github.com/kg6zjl/pick/actions/workflows/test.yml)

<p align="center">
  <img src="images/pick.jpg?raw=true" alt="Rust-y Pick" width="40%">
</p>



# Introducing Pick

Pick fills a gap in my terminal that maybe you didn't even know you had.

Pick allows you to pipe in any newline or delimiter separated data and waits for you to make your selection before passing your decision to the next tool in your piped command chain. It's a universal interactive fuzzy match choice picker for your bash script or your terminal.

Seeing as how others have developed similar concepts with the same name, we'll call this one the "Rust-y Pick".

Enjoy!

### Usage

```
$ pick --help
Pick allows you to pipe in any newline or delimiter separated data and waits
for you to make your selection before passing your decision to
the next tool in your piped command chain.

Usage: pick [OPTIONS]

Options:
  -d, --delimiter <DELIMITER>  Specify the delimiter (default is newline)
  -c, --column <COLUMN>        Specify the column
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

Have you ever wished you could do something as simple as fuzzy match pick a hostname from a list?
```
$ ssh `cat hosts.txt | pick`
›
❯ admin@pikube1
  192.168.1.100
  foobar.local
  ec2-user@111.222.333.444
```
![Pick](images/pick1b.png?raw=true "Pick")

I bet you can find all kinds of uses for pick:
```
$ cd `ls -1 | pick`
› g
❯ git
  go
  miniforge3
```
![Pick](images/pick2.png?raw=true "Pick")

You can turn just about any command into an interactive command:
```
$ alias gb="git checkout \$(git branch -a | pick)"
$ gb
·   main
M	README.md
Switched to branch 'main'
Your branch is up to date with 'origin/main'.
```
![Pick](images/pick3.png?raw=true "Pick")


### Installation
Adjust OS and ARCH for your operating system and architecture (options show on https://github.com/kg6zjl/pick/releases/latest)
```
OS="apple-darwin"
ARCH="aarch64"
LATEST_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/kg6zjl/pick/releases/latest | jq -r '.tag_name')
curl -sL -o $HOME/bin/pick https://github.com/kg6zjl/pick/releases/download/${LATEST_RELEASE}/pick-${ARCH}-${OS}
chmod +x $HOME/bin/pick
pick --version
```

### Development
Deps are `direnv` `asdf` and covered by the brew install command below. Once those are installed run `task setup` to bootstrap your local rust development environment.
```
brew install direnv asdf go-task
task setup
cd ../ && cd - && direnv allow

# verify that direnv sourced the correct rust install
which rustc
# should show $HOME/.asdf/installs/rust/x.x.x/bin/rustc
```

Before making a Pull Request, please run `task test` which will run build, tests, and integration tests.

### AI Warning
This rust code was mostly AI generated thanks to Copilot