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
            pkg-config
            sxiv
        ] ++ shared;
        shellHook = hook;
    };
}
