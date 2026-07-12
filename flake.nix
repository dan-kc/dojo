{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    ra-mux = {
      url = "github:dan-kc/ra-mux";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ra-mux,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = fenix.packages.${system}.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ];
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        scripts = import ./scripts.nix { inherit pkgs; };
      in
      {
        packages.default = rustPlatform.buildRustPackage {
          pname = "dojo";
          version = "0.1.0";
          src = ./.;
          doCheck = false;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              rustToolchain
              nil
              nixfmt
              ra-mux.packages.${system}.default
            ]
            ++ scripts;
            shellHook = ''
              status
            '';
          };
      }
    );
}
