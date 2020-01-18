# ruspline

![](cover.png)

Needed things
---
*   [Nix](https://nixos.org/nix/)

Quick start
---
```
$ ./shell
[nix-shell:path/to/ruspline]$ cd dev/
[nix-shell:path/to/ruspline/dev]$ nightrun
usage: target/release/main <alpha: f64> <tension: f64> <n_points: u8> <seed: u64> <width: u16> <height: u16> <tile_size: u16> <filename: string>
[nix-shell:path/to/ruspline/dev]$ nightrun 0.5 0.1 7 0 10 4 100 out/main.png
[nix-shell:path/to/ruspline/dev]$ open out/main.png
```
