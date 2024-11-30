{
  description = "A flake for building the yarnberry2nix Rust project";

  inputs = {
    # Use a recent version of nixpkgs that includes rustPackages.rust-src
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Import the rust-overlay for managing Rust toolchains
    rust-overlay.url = "github:oxalica/rust-overlay";

    # Import cargo2nix for converting Cargo dependencies to Nix expressions
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";

    # Import flake-utils for simplified flake configuration
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, cargo2nix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Import Nixpkgs with the rust-overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default cargo2nix.overlays.default ];
        };

        # Define the Rust toolchain using the toolchain.toml file
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

        # Generate Cargo.nix using cargo2nix
        cargoNix = cargo2nix.generate {
          projectDir = ./.;
        };

        # Create a package set for Rust using the specified toolchain
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };

        # Reference to the latest stable Rust channel
        rust = pkgs.latest.rustChannels.stable;
      in
      rec {
        packages = {
          yarnberry2nix = (rustPkgs.workspace.yarnberry2nix { });
          default = packages.yarnberry2nix;
        };

        # Development shell with the Rust toolchain and other utilities
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            toolchain # The Rust toolchain defined by toolchain.toml
            openssl # OpenSSL for cryptographic functions
            pkg-config # Helps in compiling packages
            rust-analyzer-unwrapped
          ];

          # Set environment variables
          shellHook = ''
            export RUST_SRC_PATH=${toolchain}/lib/rustlib/src/rust/library
          '';
        };
      });
}
