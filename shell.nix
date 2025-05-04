{
  system ? builtins.currentSystem,
  pkgs ? import <nixpkgs> { inherit system; },
}:
let
  libclang = pkgs.llvmPackages.libclang.lib;
in
pkgs.mkShell {
  packages = [
    pkgs.rustc
    pkgs.cargo

    pkgs.maturin

    pkgs.python3
    pkgs.uv

    pkgs.clippy
    pkgs.rust-analyzer
    pkgs.rustfmt

    pkgs.taplo
  ];

  shellHook = ''
    mkdir -p target/debug/
    ln -snf ${libclang}/lib/libclang.dylib target/debug/

    [[ -d .venv ]] && source .venv/bin/activate
  '';

  env.LIBCLANG_PATH = "${libclang}/lib";
}
