{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
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
        pkgs = import nixpkgs { inherit system overlays; };
        rust-toolchain = pkgs.rust-bin.stable."1.86.0".default;
        package = pkgs.callPackage ./nix/aos2-save-editor.nix { inherit rust-toolchain; };
      in
      {
        packages = {
          default = package.aos2-save-editor;
        };
        devShells = {
          default = package.dev-shell;
        };
      }
    );
}
