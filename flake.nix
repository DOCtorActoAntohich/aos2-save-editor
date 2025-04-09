{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust = pkgs.rust-bin.stable."1.86.0".minimal.override {
        extensions = [
	  "rustfmt"
	  "clippy"
	  "rust-analyzer"
	  "rust-src"
	];
      };
    in {
      devShells.default = pkgs.mkShell {
        packages = [
	  rust
	];
      };
    });
}
