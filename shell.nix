with import <nixpkgs> {};
let
    shared = [
        cairo
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
            sxiv
            valgrind
        ] ++ shared;
        APPEND_LIBRARY_PATH = stdenv.lib.makeLibraryPath [
            zlib
        ];
        shellHook = hook + ''
            export LD_LIBRARY_PATH="$APPEND_LIBRARY_PATH:$LD_LIBRARY_PATH"
        '';
    };
}
