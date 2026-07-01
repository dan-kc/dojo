{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        scripts = import ./scripts.nix { inherit pkgs; };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              (fenix.packages.${system}.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
                "rust-analyzer"
              ])
              nil
              nixfmt-rfc-style
              lspmux
            ]
            ++ scripts;
            shellHook = ''
              start() {
                command start
                if [ -f "$(git rev-parse --show-toplevel)/.lspmux.port" ]; then
                  export LSPMUX_PORT=$(cat "$(git rev-parse --show-toplevel)/.lspmux.port")
                fi
              }
              stop() {
                command stop
                unset LSPMUX_PORT
              }
              ROOT="$(git rev-parse --show-toplevel)"
              if [ -f "$ROOT/.lspmux.port" ] && [ -f "$ROOT/.lspmux.pid" ] && kill -0 $(cat "$ROOT/.lspmux.pid") 2>/dev/null; then
                export LSPMUX_PORT=$(cat "$ROOT/.lspmux.port")
              fi
              status
            '';
          };
      }
    );
}
