with import <nixpkgs> {};
mkShell {
    buildInputs = [
        cairo
        flamegraph
        pkg-config
        rustup
        shellcheck
    ];
    shellHook = ''
        . .shellhook
    '';
}
