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
        overlays = [
          (self: super: {
            zig = super.zig.overrideAttrs (_: {
              src = self.fetchFromGitHub {
                owner = "ziglang";
                repo = "zig";
                rev = "88d1258e08e668e620d5f8f4681315e555acbcd2";
                hash = "sha256-zNPrze2XxF+4ZwTq0LN2Y9tmPHd7lY6Nb3Cy9KN2Il8=";
              };
              patches = [
                (self.fetchpatch {
                  url = "https://github.com/ziglang/zig/pull/9771.patch";
                  sha256 = "sha256-AaMNNBET/x0f3a9oxpgBZXnUdKH4bydKMLJfXLBmvZo=";
                })
              ];
            });
          })
        ];
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
      });
      devShell = pkgs.mkShell {
        packages = with pkgs; [
          mold
          rust
          zig
        ];
      };

      # x86_64-linux-musl
      CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = pkgs.writeShellScriptBin "linker" ''
        ZIG_GLOBAL_CACHE_DIR=$(mktemp -d) zig cc -target x86_64-linux-musl -dynamic $@
      '' + /bin/linker;
      CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS = "-C target-feature=-crt-static";
      CC_x86_64_unknown_linux_musl = pkgs.writeShellScriptBin "cc" ''
        ZIG_GLOBAL_CACHE_DIR=$(mktemp -d) zig cc -target x86_64-linux-musl $@
      '' + /bin/cc;
    }
  );
}
