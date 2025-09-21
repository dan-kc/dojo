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
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        ra = pkgs.writeShellScriptBin "ra" ''
          # checks if RA_MULTIPLEX_PORT is set if not then error
          if [ -z ''${RA_MULTIPLEX_PORT} ]; then
            echo "Error: RA_MULTIPLEX_PORT is not set. Please export it before running." >&2
            exit 1
          fi
          echo "Successfully running"
          XDG_CONFIG_HOME=/home/daniel/projects/playground ra-multiplex server &> /tmp/ra-multiplex.log & disown
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (fenix.packages.${system}.complete.withComponents [
              "cargo"
              "clippy"
              "rustc"
              "rustfmt"
            ])
            rust-analyzer
            nil
            nixfmt-rfc-style
            taplo
            ra-multiplex
            ra
          ];

          shellHook = ''
            export RA_MULTIPLEX_PORT=27638
          '';
        };
      }
    );
}
