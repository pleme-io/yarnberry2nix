{
  description = "A flake for building the yarnberry2nix Rust project";

  inputs = {
    # Specify the Nixpkgs version to ensure reproducibility
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };

        # Define the Rust toolchain you want to use
        # rustChannel = pkgs.rust-bin.stable.latest.default;
      in
      {
        # Package definition for your Rust project
        # packages.yarnberry2nix = pkgs.rustPlatform.buildRustPackage {
        #   pname = "yarnberry2nix";
        #   version = "0.1.0"; # Update to your project's version
        #
        #   # Use the source code from the current directory
        #   src = self;
        #
        #   # Cargo.lock is used for reproducibility
        #   cargoLock = {
        #     lockFile = self + /Cargo.lock;
        #   };
        #
        #   # Fetch the crates.io index; the hash will be auto-computed
        #   # Alternatively, you can set cargoSha256 to "0000000000000000000000000000000000000000000000000000"
        #   # and Nix will provide the correct hash on build failure
        #   cargoSha256 = "0000000000000000000000000000000000000000000000000000";
        #
        #   # Specify the Rust toolchain
        #   rustc = rustChannel.rustc;
        #   cargo = rustChannel.cargo;
        #
        #   # Include any native build inputs your project requires
        #   nativeBuildInputs = with pkgs; [
        #     # For example, if you need OpenSSL:
        #     # openssl
        #   ];
        # };

        # Development shell with the Rust toolchain and other utilities
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustChannel.rustc
            rustChannel.cargo
            # pkgs.pkg-config
            # Add other tools you need in your development environment
          ];
        };
      });
}

