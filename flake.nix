{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: let
      rust = fenix.packages.${system};
      toolchain = rust.stable;
    in {
      defaultPackage = (naersk.lib.${system}.override {
        inherit (toolchain) cargo rustc;
      }).buildPackage {
        src = ./.;
      };

      devShell = let
        pkgs = nixpkgs.legacyPackages.${system};
      in pkgs.mkShell {
        nativeBuildInputs = [
          toolchain.cargo
          toolchain.rustc
          rust.rust-analyzer
        ];
      };
    });
}
