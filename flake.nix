# Inspired by https://log.woodweb.ca/articles/rust-flake/
{
  description = "Rust GUI Setup";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      # Standard nixpkgs with a rust overlay
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
        config = { };
      };
      runtimeDeps = [ ];
      buildDeps = [
        pkgs.pkg-config
        pkgs.rustPlatform.bindgenHook
      ];
      devDeps = [
        pkgs.gdb
      ];

      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      # MSRV = Minimum Supported Rust Version
      msrv = cargoToml.package.rust-version;

      rustPackage =
        features:
        (pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.minimal;
          rustc = pkgs.rust-bin.stable.latest.minimal;
        }).buildRustPackage
          {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildFeatures = features;
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps;
            # Uncomment if your cargo tests require networking or otherwise
            # don't play nicely with the Nix build sandbox:
            # doCheck = false;
          };

      mkDevShell =
        rustc:
        pkgs.mkShell {
          shellHook = ''
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
          buildInputs = runtimeDeps;
          nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
        };
    in
    {
      # Setup command line operations
      # nix run '.#example'
      packages.x86_64-linux.example = rustPackage "foobar";
      # nix run '.#example-base'
      packages.x86_64-linux.example-base = rustPackage "";
      # nix develop
      devShells.nightly = mkDevShell (
        pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
      );
      devShells.x86_64-linux.default = self.devShells.nightly;
      # nix develop '.#stable'
      devShells.stable = mkDevShell pkgs.rust-bin.stable.latest.default;
      # nix develop '.#msrv'
      devShells.msrv = mkDevShell pkgs.rust-bin.stable.${msrv}.default;

      # TODO: Have nix run run stable program
      packages.x86_64-linux.default = self.pkgs.hello;
    };
}
