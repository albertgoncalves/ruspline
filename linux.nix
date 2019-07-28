{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "ruspline";
    buildInputs = [
        cairo
        pkg-config
        rustup
        shellcheck
    ];
    shellHook = ''
        . .shellhook
    '';
}
