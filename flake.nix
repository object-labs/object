{
  description = "Object development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";

    keel = {
      url = "github:spoke-sh/keel";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = {
    self,
    flake-utils,
    keel,
    nixpkgs,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        object = rustPlatform.buildRustPackage {
          pname = "object";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        keelCli = keel.packages.${system}.default;
      in {
        packages = {
          default = object;
          keel = keelCli;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = object;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.just
            pkgs.pkg-config
            pkgs.cargo-nextest
            keelCli
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      });
}
