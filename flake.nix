{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

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

    fenix.url = "github:nix-community/fenix";

    naersk.url = "github:Naxdy/naersk?ref=work/consider-additional-cargo-lock";

    linkle-src = {
      url = "github:MegatonHammer/linkle";
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
    , fenix
    , naersk
    , linkle-src
    }: (flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
          fenix.overlays.default
        ];
      };

      skyline-rust-src = pkgs.runCommandLocal "skyline-rust-src"
        {
          src = pkgs.fetchFromGitHub {
            owner = "skyline-rs";
            repo = "rust-src";
            rev = "3b1dd4aca19b5dca6e90a3de457304f013d7bc77";
            hash = "sha256-aXmyW7pVSar31r9nm+WvmKQcoXI5Ht3ZEnB3yWbnxJY=";
            fetchSubmodules = true;
          };
        } ''
        mkdir -p $out/lib/rustlib/src/
        cp -r $src $out/lib/rustlib/src/rust
      '';

      nightlyToolchain = pkgs.fenix.toolchainOf {
        channel = "nightly";
        date = "2023-12-30";
        sha256 = "sha256-6ro0E+jXO1vkfTTewwzJu9NrMf/b9JWJyU8NaEV5rds=";
      };

      stableToolchain = pkgs.fenix.stable.withComponents [
        "cargo"
        "rustc"
      ];

      skylineToolchain = fenix.packages.${system}.combine ((builtins.attrValues {
        inherit (nightlyToolchain)
          cargo
          clippy
          rustc
          rustfmt;
      }) ++ [
        skyline-rust-src
      ]);

      naersk_skyline = naersk.lib.${system}.override {
        cargo = skylineToolchain;
        rustc = skylineToolchain;
      };

      naersk_stable = naersk.lib.${system}.override {
        cargo = stableToolchain;
        rustc = stableToolchain;
      };

      patched-build-target = pkgs.runCommandLocal "skyline-build-target"
        {
          src = pkgs.substitute {
            src = "${skyline-rs}/aarch64-skyline-switch.json";
            substitutions = [
              "--replace-fail"
              "-Tlink.T"
              "-T${cargo-skyline-src}/src/link.T"
            ];
          };
        } ''
        mkdir -p $out
        cp $src $out/aarch64-skyline-switch.json
      '';

      CARGO_BUILD_TARGET = "${patched-build-target}/aarch64-skyline-switch.json";
    in
    {
      packages = {
        default = self.packages.${system}.latency-slider-de-nro;

        latency-slider-de-nro = pkgs.runCommandLocal "latency-slider-de-nro-${self.packages.${system}.latency-slider-de.version}" { } ''
          mkdir -p $out/lib
          ${self.packages.${system}.linkle}/bin/linkle nro ${self.packages.${system}.latency-slider-de}/lib/liblatency_slider_de.so $out/lib/liblatency_slider_de.nro
        '';

        latency-slider-de =
          let
            cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
          naersk_skyline.buildPackage {
            pname = cargoTOML.package.name;
            version = cargoTOML.package.version;

            src = self;

            additionalCargoLock = "${skyline-rust-src}/lib/rustlib/src/rust/Cargo.lock";

            cargoBuildOptions = old: old ++ [
              "-Z"
              "build-std=core,alloc,std,panic_abort"
            ];

            copyLibs = true;
            copyBins = false;

            gitSubmodules = true;

            env = {
              inherit CARGO_BUILD_TARGET;
              SKYLINE_ADD_NRO_HEADER = "1";
            };
          };

        linkle =
          let
            cargoTOML = builtins.fromTOML (builtins.readFile "${linkle-src}/Cargo.toml");
          in
          naersk_stable.buildPackage {
            pname = cargoTOML.package.name;
            version = cargoTOML.package.version;

            src = linkle-src;

            # https://github.com/nix-community/naersk/issues/127
            singleStep = true;

            cargoBuildOptions = old: old ++ [
              "--bin"
              "linkle"
              "--features=binaries"
            ];
          };
      };


      devShells.default = pkgs.mkShell {
        nativeBuildInputs = builtins.attrValues {
          inherit (pkgs) gdb python311;
          inherit skylineToolchain;
        };

        inherit CARGO_BUILD_TARGET;
      };
    }));
}
