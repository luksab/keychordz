let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  # pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz")) {};

  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball ("channel:nixpkgs-unstable")) { };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    clippy
    rust-analyzer
    rustup
    rustfmt

    pkgsCross.avr.buildPackages.gcc
    pkgsCross.avr.buildPackages.binutils
    pkgsCross.avr.avrlibc
    avrdude

    pkg-config
    openssl
    clang
    udev
  ];

  AVR_CPU_FREQUENCY_HZ = 16000000;

  LD_LIBRARY_PATH = with pkgs;
    "${udev}/lib";

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
