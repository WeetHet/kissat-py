{
  system ? builtins.currentSystem,
  pkgs ? import <nixpkgs> { inherit system; },
}:
pkgs.python3.pkgs.callPackage ./package.nix { }
