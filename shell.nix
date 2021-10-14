let
  overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ overlay ]; };
  rust-bin = pkgs.rust-bin.stable.latest.minimal.override {
    extensions = [ "clippy" "rustfmt" "llvm-tools-preview" ];
  };

  # https://git.m-labs.hk/M-Labs/ENC424J600/commit/d399ebc3df7a5b61745a8eee86e9e68d1eb7ff9b
  itm = pkgs.rust.packages.stable.rustPlatform.buildRustPackage rec {
    version = "0.3.1";
    pname = "itm";

    src = pkgs.fetchFromGitHub {
      owner = "rust-embedded";
      repo = "itm";
      rev = "v${version}";
      sha256 = "15pa0ydm19vz8p3wairpx3vqzc55rp4lgki143ybgw44sgf8hraj";
    };

    cargoSha256 = "1akcpminjdzkg7a84cdqjrd1hq4rs0fcx1z95zq2bfr3xvqj0jyh";

    cargoPatches = [ ./itm-cargo-lock.patch ];

    nativeBuildInputs = [ pkgs.pkg-config ];

    doCheck = false;
  };
in pkgs.mkShell {
  buildInputs = [
    itm
    rust-bin

    pkgs.cargo-binutils
    pkgs.cargo-edit
    # pkgs.gcc-arm-embedded
    pkgs.minicom
    pkgs.openocd
    pkgs.pkg-config
    pkgs.rust-analyzer
  ];

  shellHook = "";
}
