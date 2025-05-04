{
  lib,
  buildPythonPackage,
  rustPlatform,
  llvmPackages,
  nix-gitignore,
}:

buildPythonPackage rec {
  pname = "kissat-py";
  version = "0.1.0";
  pyproject = true;

  src = nix-gitignore.gitignoreSource [ ] ./.;

  cargoDeps = rustPlatform.fetchCargoVendor {
    inherit pname version src;
    hash = "sha256-zaMXQOLjAsGlEepDbmM2/ab9ov5rKRKXY1puchHcWtw=";
  };

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
    rustPlatform.maturinBuildHook
  ];

  env.LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";

  meta = {
    maintainers = [ lib.maintainers.WeetHet ];
  };
}
