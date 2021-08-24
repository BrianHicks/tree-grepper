{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-21.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.tree-grepper =
            let darwinInputs = if pkgs.stdenv.isDarwin then [ pkgs.xcbuild ] else [ ];
            in naersk-lib.buildPackage {
              root = ./.;
              buildInputs = [ pkgs.libiconv pkgs.rustPackages.clippy ] ++ darwinInputs;

              doCheck = true;
              checkPhase = ''
                cargo test
                cargo clippy -- --deny warnings
              '';
            };
          defaultPackage = packages.tree-grepper;

          # `nix run`
          apps.tree-grepper = flake-utils.lib.mkApp {
            drv = packages.tree-grepper;
          };
          defaultApp = apps.tree-grepper;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    );
}
