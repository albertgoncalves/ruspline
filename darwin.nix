{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        cairo
        gtk2
        rustup
    ];
    shellHook = ''
        . .shellhook
    '';
}
