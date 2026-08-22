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

  cargoHash = "sha256-tIUifdvQT/WYJwOwNmXl6hLtsjdE0LEBJ5BgOQ2VCZg=";

  meta = with lib; {
    description = toml.workspace.package.description;
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
