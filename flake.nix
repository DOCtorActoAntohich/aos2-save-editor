{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      nix-filter,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        pkgs-windows = pkgs.pkgsCross.mingwW64;

        rust-version = "1.93.0";
        rust = {
          dev = pkgs.rust-bin.stable.${rust-version}.minimal.override {
            extensions = [
              "rustfmt"
              "clippy"
              "rust-analyzer"
              "rust-src"
            ];
          };
          ci = pkgs.rust-bin.stable.${rust-version}.minimal.override {
            extensions = [
              "rustfmt"
              "clippy"
            ];
          };
          build = pkgs.rust-bin.stable.${rust-version}.minimal;
          windows =
            let
              rust-bin = rust-overlay.lib.mkRustBin { } pkgs-windows.buildPackages;
            in
            rust-bin.stable.${rust-version}.minimal;
        };

        aos2-save-editor = pkgs.callPackage ./nix/aos2-save-editor.nix {
          inherit nix-filter;
          rust-toolchain = rust.build;
        };
        aos2-save-editor-windows = pkgs-windows.callPackage ./nix/aos2-save-editor.nix {
          inherit nix-filter;
          rust-toolchain = rust.windows;
        };
      in
      {
        formatter = pkgs.nixfmt-tree;

        packages = {
          inherit aos2-save-editor;
          default = aos2-save-editor;
          windows = aos2-save-editor-windows;
        };

        devShells = {
          default = pkgs.mkShell {
            packages = [ rust.dev ];
          };
          ci = pkgs.mkShell {
            packages = [ rust.ci ];
          };
        };
      }
    );
}
