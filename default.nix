{ sources ? import ./nix/sources.nix { }, pkgs ? import sources.nixpkgs { }, ...
}:
let
  naersk = pkgs.callPackage sources.naersk { };
  gitignore = import sources.gitignore { };
  darwinInputs = if pkgs.stdenv.isDarwin then [ pkgs.xcbuild ] else [ ];
in naersk.buildPackage {
  src = gitignore.gitignoreSource ./.;
  buildInputs = [ pkgs.libiconv ] ++ darwinInputs;
}
