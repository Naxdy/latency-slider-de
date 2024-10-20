{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    flake-utils.url = "github:numtide/flake-utils";

    nix-skyline-rs.url = "github:Naxdy/nix-skyline-rs";
    nix-skyline-rs.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { self
    , flake-utils
    , nix-skyline-rs
    , nixpkgs
    }: (flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      packages = {
        default = self.packages.${system}.latency-slider-de;

        latency-slider-de =
          let
            cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
          pkgs.callPackage
            ({ mode ? "build", copyLibs ? true, cargoBuildOptions ? old: old, overrideMain ? old: old }:
              nix-skyline-rs.lib.${system}.mkNroPackage {
                pname = cargoTOML.package.name;
                version = cargoTOML.package.version;
                src = self;

                inherit cargoBuildOptions overrideMain mode copyLibs;
              })
            { };

        latency-slider-de-classic = pkgs.callPackage
          ({ mode ? "build", copyLibs ? true }: self.packages.${system}.latency-slider-de.override {
            inherit mode copyLibs;

            cargoBuildOptions = old: old ++ [
              "--no-default-features"
            ];

            overrideMain = old: {
              postInstall = (if (old ? postInstall) && (old.postInstall != false) then old.postInstall else "") + (pkgs.lib.optionalString (mode == "build") ''
                mv $out/lib/liblatency_slider_de.nro $out/lib/liblatency_slider_de_classic.nro
              '');
            };
          })
          { };
      };

      checks = {
        clippy = self.packages.${system}.latency-slider-de.override {
          mode = "clippy";
        };

        clippy-classic = self.packages.${system}.latency-slider-de-classic.override {
          mode = "clippy";
        };
      };

      devShells.default = nix-skyline-rs.devShells.${system}.default;
    }));
}
