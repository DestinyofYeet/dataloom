{
  rustPlatform,
  lib,
  pkgs,
  toml,
  ...
}:
let
  deps = import ./dependencies.nix { inherit pkgs; };
in

rustPlatform.buildRustPackage {
  pname = "dataloom";
  version = toml.workspace.package.version;

  buildInputs = deps.packages;

  src = ../.;

  cargoHash = "sha256-AhisLVfAJ+PbG13ebBaxTdOJj5CykVrLcYkvPuAU1Bw=";

  meta = with lib; {
    description = toml.workspace.package.description;
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
