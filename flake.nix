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
      pkgs = nixpkgs.legacyPackages.${system};
      # TODO: figure out how to read this from the toochain file directly
      toolchain = fenix.packages.${system}.toolchainOf {
        channel = "1.59";
        sha256 = "sha256-4IUZZWXHBBxcwRuQm9ekOwzc0oNqH/9NkI1ejW7KajU=";
      };
    in {
      defaultPackage = (naersk.lib.${system}.override {
        inherit (toolchain) cargo rustc;
      }).buildPackage {
        src = ./.;
      };

      devShell = pkgs.mkShell {
        nativeBuildInputs = [ (toolchain.withComponents [
          "cargo" "rustc" "rust-src" "rustfmt" "clippy"
        ])];
      };
    });
}
