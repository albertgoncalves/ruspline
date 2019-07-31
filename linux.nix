with import <nixpkgs> {};
mkShell {
    buildInputs = [
        cairo
        flamegraph
        glibcLocales
        linuxPackages.perf
        pkg-config
        rustup
        shellcheck
    ];
    shellHook = ''
        . .shellhook
    '';
}
