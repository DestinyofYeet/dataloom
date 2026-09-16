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
  pname = toml.package.name;
  version = toml.workspace.package.version;

  buildInputs = deps.packages;

  src = ../.;

  cargoHash = "sha256-ULi3cMK/v3r2v6SfjjkBAySbduegAC/w/zRlqz3cv70=";

  meta = with lib; {
    description = toml.workspace.package.description;
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
