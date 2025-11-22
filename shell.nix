{
  pkgs ? import <nixpkgs> {
    overlays = [

      # Overlay to configure toolchain for Rust
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/stable.tar.gz"))
    ];
  },

  # TODO: Remove unstable espflash upon stable working release
  # espflash==3.3.0 does not correctly flash
  unstable ? import <nixos-unstable> { },
}:

let
  rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell {

  packages = with pkgs; [ rust-bin ] ++ [ unstable.espflash ];
}
