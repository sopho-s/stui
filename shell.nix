{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
        doxygen
        act
        docker
        cargo
        rustc
        SDL2
        gdb
    ];
}