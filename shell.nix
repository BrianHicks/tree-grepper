{ ... }:
let
  sources = import ./nix/sources.nix;
  nixpkgs = import sources.nixpkgs { };
  niv = import sources.niv { };
in with nixpkgs;
stdenv.mkDerivation {
  name = "tree-grepper";
  buildInputs = [
    niv.niv
    git

    # tree-sitter C deps
    pkgs.libiconv

    # rust tools
    cargo
    cargo-insta
    cargo-watch
    rustPackages.clippy
    rustc
    rustfmt
  ];
}
