{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
    gitignore = {
      url = "github:hercules-ci/gitignore";
      flake = false;
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs, gitignore, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        inherit (import gitignore { inherit (pkgs) lib; }) gitignoreSource;

        # Get a specific rust version
        mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") { };
        chanspec = {
          date = "2021-03-31";
          channel = "nightly";
          sha256 = "oK5ebje09MRn988saJMT3Zze/tRE7u9zTeFPV1CEeLc="; # set zeros after modifying channel or date
        };

        rustChannel = mozilla.rustChannelOf chanspec;
        rust = rustChannel.rust;
        rust-src = rustChannel.rust-src;

        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };

        nativeBuildInputs = with pkgs; [
          SDL2
          SDL2_ttf
        ];

      in
      rec {
        packages.my-project = naersk-lib.buildPackage {
          pname = "obiv";
          version = "0.1.0";
          root = ./.;
          inherit nativeBuildInputs;
          cargoBuildOptions =
            v:
            [ "$cargo_release" ''-j "$NIX_BUILD_CORES"'' "--out-dir" "out" "--all-features" "--bin" "obiv" "--message-format=$cargo_message_format" ];
        };
        defaultPackage = packages.my-project;
        apps.my-project = utils.lib.mkApp {
          drv = packages.my-project;
        };
        defaultApp = apps.my-project;
        devShell = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs ++ [
            rust
            rust-src
            pkgs.rust-analyzer
            pkgs.rustfmt
            pkgs.cargo
          ];
          RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
          RUST_LOG = "info";
          RUST_BACKTRACE = 1;
        };
      });
}
