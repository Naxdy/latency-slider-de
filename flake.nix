{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";

    cargo-skyline-src = {
      url = "github:jam1garner/cargo-skyline";
      flake = false;
    };

    skyline-rs = {
      url = "github:ultimate-research/skyline-rs";
      flake = false;
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , cargo-skyline-src
    , skyline-rs
    }: (flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };

      rustToolchain = (pkgs.rust-bin.nightly."2023-12-30".default.override {
        extensions = [
          "rust-src"
          "rust-std"
        ];
      });
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

      cargo-skyline = rustPlatform.buildRustPackage {
        pname = "cargo-skyline";
        version = (builtins.fromTOML (builtins.readFile "${cargo-skyline-src}/Cargo.toml")).package.version;

        src = cargo-skyline-src;

        cargoLock = {
          lockFile = "${cargo-skyline-src}/Cargo.lock";
        };

        meta = {
          description = "A cargo subcommand for working with Skyline plugins written in Rust";
        };
      };
    in
    {
      packages.rustToolchain = rustToolchain;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = builtins.attrValues {
          inherit cargo-skyline;

          # for now, we use rustup to manage the toolchain
          # because skyline has some weird customizations that
          # I don't feel like getting into at this point in time
          # this means no reproducible builds (for now), but I can live with that
          inherit (pkgs) rustup;
        };

        # this is mainly for rust-analyzer to not get confused
        CARGO_BUILD_TARGET = "${skyline-rs}/aarch64-skyline-switch.json";
      };
    }));
}
