{ sources ? import ./nix/sources.nix { }, pkgs ? import sources.nixpkgs { }, ...
}:
let
  naersk = pkgs.callPackage sources.naersk { };
  gitignore = import sources.gitignore { };
in naersk.buildPackage {
  src = gitignore.gitignoreSource ./.;
  buildInputs = if pkgs.stdenv.isDarwin then [ pkgs.xcbuild ] else [ ];
}
