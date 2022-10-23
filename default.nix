let
  pkgs =
    import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc];
  };
in with pkgs;
mkShell {
  name = "scriptr";
  buildInputs = [rust-toolchain];
  RUST_BACKTRACE = 1;
}

