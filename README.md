# ruspline

![](cover.png)

Needed things
---
 * [Nix](https://nixos.org/nix/)

Quick start
---
```
$ ./shell
[nix-shell:path/to/ruspline]$ cd dev/
[nix-shell:path/to/ruspline/dev]$ nightrun
usage: target/release/main <width: int> <height: int> <seed: int> <filename: string>
[nix-shell:path/to/ruspline/dev]$ nightrun 15 6 1 out/main.png
[nix-shell:path/to/ruspline/dev]$ open out/main.png
```
