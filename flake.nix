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
    let
      nix-filter-overlay = final: prev: {
        nix-filter = nix-filter.lib;
      };
      overlays = [
        nix-filter-overlay
        (import rust-overlay)
      ];
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        rust-version = "1.93.0";

        pkgs = import nixpkgs { inherit system overlays; };
        pkgs-windows = pkgs.pkgsCross.mingwW64;

        rust-toolchain = pkgs.rust-bin.stable.${rust-version}.default;
        rust-toolchain-windows =
          let
            rust-bin = rust-overlay.lib.mkRustBin { } pkgs-windows.buildPackages;
          in
          rust-bin.stable.${rust-version}.minimal;

        aos2-save-editor = pkgs.callPackage ./nix/aos2-save-editor.nix { inherit rust-toolchain; };
      in
      {
        packages = {
          default = aos2-save-editor;
          windows = pkgs-windows.callPackage ./nix/aos2-save-editor.nix {
            rust-toolchain = rust-toolchain-windows;
          };
        };
        devShells = {
          default = pkgs.mkShell {
            inputsFrom = [ aos2-save-editor ];
          };
        };
      }
    );
}
