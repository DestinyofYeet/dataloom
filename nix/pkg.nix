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

  cargoHash = "sha256-J4/WwjAoX/1LAbU99c5AjlrWbmkJROGTJ0fn1+3Cv/M=";

  meta = with lib; {
    description = toml.workspace.package.description;
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
