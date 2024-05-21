# Introducing Pick

Pick fills a gap in my terminal that maybe you didn't even know you had.

Have you ever wished you could do something as simple as pick a hostname from a list?
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


Pick allows you to pipe in any newline separated data and waits for you to make your selection before passing your decision to the next tool in your piped command chain.

Enjoy!

### AI Warning
This rust code was mostly AI generated thanks to Copilot