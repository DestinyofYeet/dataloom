{ pkgs, ... }:
{
  packages = with pkgs; [
    rustc
    cargo
    clippy
    # openssl
    # pkg-config
    rust-analyzer
    rustfmt # formatter
    sqlite.dev
    cargo-expand
    toml-cli
    cargo-deny
    cargo-audit
    commitizen
  ];
}
