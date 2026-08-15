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
  version = toml.package.version;

  buildInputs = deps.packages;

  src = ../.;

  cargoHash = "sha256-5Fs91FQqm32km8Tz82MzvZAC0Z2z60xDUykAWhSQdTk=";

  meta = with lib; {
    description = toml.package.description;
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
