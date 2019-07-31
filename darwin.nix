with import <nixpkgs> {};
mkShell {
    buildInputs = [
        cairo
        flamegraph
        gtk2
        rustup
        shellcheck
    ];
    shellHook = ''
        . .shellhook
    '';
}
