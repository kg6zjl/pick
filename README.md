# Introducing Pick

Pick fills a gap in my terminal that maybe you didn't even know you had.

Have you ever wished you could do something as simple as pick a hostname from a list?
```
$ cat hosts.txt | pick | xargs -I {} ssh -tt {}
· 192.168.32.124
```
![Pick](images/pick1.png?raw=true "Pick")

Pick allows you to pipe in any newline separated data and waits for you to make your selection before passing your decision to the next tool in your piped command chain.

Enjoy!

### AI Warning
This rust code was mostly AI generated thanks to Copilot