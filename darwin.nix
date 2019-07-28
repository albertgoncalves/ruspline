{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
    name = "ruspline";
    buildInputs = [
        cairo
        gtk2
        rustup
        shellcheck
    ];
    shellHook = ''
        . .shellhook
    '';
}
