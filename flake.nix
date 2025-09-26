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

        ra-multiplex-port = "27610"; # CHANGE
        ra-config = ''
          instance_timeout = false 
          gc_interval = 10
          listen = ["127.0.0.1", ${ra-multiplex-port}]
          connect = ["127.0.0.1", ${ra-multiplex-port}]
          log_filters = "info"
          pass_environment = []
        '';
        ra = pkgs.writeShellScriptBin "ra" ''
          RA_MULTIPLEX_DIR="/tmp/ra-${ra-multiplex-port}"
          CONFIG_DIR="$RA_MULTIPLEX_DIR/ra-multiplex"  
          CONFIG_FILE="$CONFIG_DIR/config.toml"
          LOG_DIR="/tmp/ra-multiplex"
          LOG_FILE="$LOG_DIR/$RA_MULTIPLEX_PORT.log"

          mkdir -p "$LOG_DIR"
          mkdir -p "$CONFIG_DIR"
          cat > "$CONFIG_FILE" <<EOF
          ${ra-config}
          EOF

          XDG_CONFIG_HOME=$RA_MULTIPLEX_DIR ra-multiplex server &> "$LOG_FILE" & disown
          echo "Listening"
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
            export RA_MULTIPLEX_PORT="${ra-multiplex-port}"
          '';
        };
      }
    );
}
