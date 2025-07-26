{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    let
      overlays = [
        (import rust-overlay)
      ];
      supported-systems = [
        "x86_64-linux"
      ];
      mk-rust-platform = { pkgs, ... }: pkgs.rust-bin.stable."1.86.0".default;
      mk-dev-shell = { pkgs, rust-platform, ... }: pkgs.mkShell { packages = [ rust-platform ]; };
      mk-build =
        {
          nixpkgs,
          system,
          overlays,
          ...
        }:
        let
          pkgs = import nixpkgs { inherit system overlays; };
          rust-platform = mk-rust-platform { inherit pkgs; };
          dev-shell = mk-dev-shell { inherit pkgs rust-platform; };
        in
        {
          inherit dev-shell;
        };
    in
    {
      devShells = nixpkgs.lib.genAttrs supported-systems (
        system:
        let
          build = mk-build { inherit nixpkgs system overlays; };
        in
        {
          default = build.dev-shell;
        }
      );
    };
}
