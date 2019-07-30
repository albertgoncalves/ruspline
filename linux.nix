with import <nixpkgs> {};
mkShell {
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
