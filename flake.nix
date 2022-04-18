{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    fenix = {
      url = "github:nix-community/fenix";
    };
  };
  outputs = inputs: inputs.flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import inputs.nixpkgs {
        inherit system;
      };
      rust = inputs.fenix.packages.${system}.fromToolchainFile {
        dir = ./.;
        sha256 = "sha256-otgm+7nEl94JG/B+TYhWseZsHV1voGcBsW/lOD2/68g=";
      };
    in
    rec {
      defaultApp = inputs.flake-utils.lib.mkApp {
        drv = defaultPackage;
      };
      defaultPackage = (pkgs.makeRustPlatform {
        rustc = rust;
        cargo = rust;
      }).buildRustPackage ({
        name = "deciduously_com";
        src = ./.;
        doCheck = false;
        cargoLock = { lockFile = ./Cargo.lock; };
        cargoBuildFlags = "--package deciduously_com";
      });
      devShell = pkgs.mkShell {
        packages = with pkgs; [
          rust
        ];
      };
    }
  );
}
