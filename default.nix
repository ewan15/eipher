let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
nixpkgs.mkShell {
  name = "scriptr";
  buildInputs = [ (nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust ];
  RUST_BACKTRACE = 1;
}
