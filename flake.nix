{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nmattia/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        darwinInputs = if pkgs.stdenv.isDarwin then [ pkgs.xcbuild ] else [ ];
      in rec {
        # `nix build`
        packages.tree-grepper = naersk-lib.buildPackage {
          root = ./.;
          buildInputs = [ pkgs.libiconv pkgs.rustPackages.clippy ]
            ++ darwinInputs;

          doCheck = true;
          checkPhase = ''
            cargo test
            cargo clippy -- --deny warnings
          '';
        };
        defaultPackage = packages.tree-grepper;
        overlay = final: prev: { tree-grepper = packages.tree-grepper; };

        # `nix run`
        apps.tree-grepper =
          flake-utils.lib.mkApp { drv = packages.tree-grepper; };
        defaultApp = apps.tree-grepper;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs;
            [
              cargo
              cargo-edit
              cargo-insta
              # https://github.com/NixOS/nixpkgs/issues/146349
              # cargo-watch
              rustPackages.clippy
              rustc
              rustfmt

              # for some reason this seems to be required, especially on macOS
              libiconv
            ] ++ darwinInputs;
        };
      });
}
