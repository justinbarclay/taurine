{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        libraries = with pkgs;[
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
        ];

        packages = with pkgs; [
          materia-theme
          curl
          wget
          pkg-config
          dbus
          openssl_3
          glib
          gtk3
          libsoup
          webkitgtk
          librsvg
          materia-theme
        ];

        target = "wasm32-unknown-unknown";

        toolchain = with fenix.packages.${system}; combine [
          minimal.cargo
          minimal.rustc
          targets.${target}.latest.rust-std
        ];

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          buildInputs = [ pkgs.cargo-nextest packages ];
          src = ./.;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = libraries ++ packages ++ [ toolchain ];
          buildInputs = with pkgs; [ cargo-nextest webkitgtk librsvg rust-analyzer rustfmt ];
          shellHook =
            ''
              export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH:${pkgs.materia-theme}/share
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
            '';
        };
      }
    );
}
