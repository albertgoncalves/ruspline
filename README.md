# ruspline

![](cover.png)

Needed hings
---
 * [Nix](https://nixos.org/nix/)

Quick start
---
```
$ ./shell
[nix-shell:path/to/ruspline]$ cd dev/
[nix-shell:path/to/ruspline/dev]$ nightrun
usage: target/release/main <width: int> <height: int> <seed: int>
[nix-shell:path/to/ruspline/dev]$ nightrun 15 6 1 && open out/main.png
```
