{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "Rust";
    buildInputs = [
        cairo
        pkg-config
        rustup
    ];
    shellHook = ''
        . .shellhook
    '';
}
