with import <nixpkgs> {};
mkShell {
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
