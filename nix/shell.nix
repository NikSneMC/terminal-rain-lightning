{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
      ];
    },
}: let
  packages = with pkgs; [
    (rust-bin.nightly.latest.default.override {
      extensions = [ "rust-src" ];
    })

    watchexec
    cargo-udeps
    cargo-audit
    cargo-expand

  ];

  libraries = with pkgs; [
    pkg-config
    ncurses
  ];
in
  with pkgs;
    mkShell {
      name = "minizord";
      buildInputs = packages ++ libraries;

      DIRENV_LOG_FORMAT = "";
      LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
    }
