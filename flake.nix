{
  description = "Provide macros for serializing and deserializing XML";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-analyzer" "rust-src" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Required for tests
        xmlFilter = path: _type: builtins.match ".*xml$" path != null;
        snapshotTestFilter = path: _type: builtins.match ".*snap" path != null;

        # Required for including the README files in the documentation
        readmeFilter = path: _type: builtins.match ".*/README.md$" path != null;

        srcFilter = path: type: (xmlFilter path type)
          || (snapshotTestFilter path type)
          || (readmeFilter path type)
          || (craneLib.filterCargoSources path type);

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = srcFilter;
        };

        commonArgs = {
          inherit src;

          pname = "ex_em_ell";
          version = "0.1.0";
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          dummySrc = craneLib.mkDummySrc {
            inherit src;

            # ex_em_ell_derive is a proc macro crate, so it cannot have non-proc macro functions
            extraDummyScript = ''
              rm $out/ex_em_ell_derive/src/lib.rs
              touch $out/ex_em_ell_derive/src/lib.rs
            '';
          };
        });

        ex_em_ell = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      rec {
        checks = {
          inherit ex_em_ell;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          fmt = craneLib.cargoFmt (commonArgs // {
            inherit src;
          });
        };

        packages.ex_em_ell = ex_em_ell;
        packages.default = packages.ex_em_ell;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};

          packages = with pkgs; [
            rustToolchain
            cargo-edit
            cargo-expand
            cargo-insta
            cargo-msrv
            cargo-outdated
            cargo-release

            # GitHub tooling
            gh

            # Nix tooling
            nixpkgs-fmt
          ];
        };
      });
}
