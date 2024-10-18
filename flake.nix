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
    , ...
    }: (flake-utils.lib.eachDefaultSystem (system: {
      packages = {
        default = self.packages.${system}.latency-slider-de;

        latency-slider-de =
          let
            cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
          nix-skyline-rs.lib.${system}.mkNroPackage {
            pname = cargoTOML.package.name;
            version = cargoTOML.package.version;
            src = self;
          };
      };

      devShells.default = nix-skyline-rs.devShells.${system}.default;
    }));
}
