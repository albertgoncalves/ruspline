with import <nixpkgs> {};
let 
    shared = [
        cairo
        flamegraph
        rustup
        shellcheck
    ];
    hook = ''
        . .shellhook
    '';
in
{
    darwin = mkShell {
        buildInputs = [
            gtk2
        ] ++ shared;
        shellHook = hook;
    };
    linux = mkShell {
        buildInputs = [
            glibcLocales
            linuxPackages.perf
            pkg-config
        ] ++ shared;
        shellHook = hook;
    };
}
