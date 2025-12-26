{
  pkgs ? import <nixpkgs> {
    overlays = [

      # Overlay to configure toolchain for Rust
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/stable.tar.gz"))
    ];
  },
}:

let
  rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell {

  packages = with pkgs; [ rust-bin ] ++ [ espflash ];
}
