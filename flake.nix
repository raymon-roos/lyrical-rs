{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    allSystems = ["x86_64-linux"];

    forAllSystems = f:
      nixpkgs.lib.genAttrs allSystems (system:
        f {
          pkgs = import nixpkgs {inherit system;};
        });

    manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
  in {
    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          # compile-time dependencies
          pkg-config
          rustc
          cargo
          rust-analyzer
          rustfmt
          clippy
          bacon
        ];

        buildInputs = [pkgs.openssl]; # runtime dependencies
      };
    });

    packages = forAllSystems ({pkgs}: {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = manifest.name;
        version = manifest.version;
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;

        nativeBuildInputs = [pkgs.pkg-config]; # compile-time dependencies
        buildInputs = [pkgs.openssl]; # runtime dependencies
      };
    });
  };
}
