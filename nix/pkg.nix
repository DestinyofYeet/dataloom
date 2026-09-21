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

  cargoHash = "sha256-63RIo4H4CUBY4qKuUG/hW4TfN73HHHNv2NjGAICQP9Y=";

  meta = with lib; {
    description = toml.workspace.package.description;
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
