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

    # rust tools
    cargo
    cargo-watch
    rustPackages.clippy
    rustc
    rustfmt
  ];
}
