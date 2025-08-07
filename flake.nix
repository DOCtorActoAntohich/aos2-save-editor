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
      overlays = [
        (import rust-overlay)
      ];
      supported-systems = [
        "x86_64-linux"
      ];

      mk-rust =
        {
          pkgs,
        }:
        pkgs.rust-bin.stable."1.86.0".default;

      mk-rust-platform =
        { pkgs }:
        let
          rust = mk-rust { inherit pkgs; };
        in
        pkgs.makeRustPlatform {
          rustc = rust;
          cargo = rust;
        };

      read-package-version =
        { cargo-toml }:
        let
          manifest = builtins.fromTOML (builtins.readFile cargo-toml);
        in
        manifest.workspace.package.version;

      mk-editor =
        { pkgs, rust-platform }:
        let
          crate-version = read-package-version { cargo-toml = ./Cargo.toml; };
        in
        rust-platform.buildRustPackage {
          pname = "aos2-save-editor";
          version = crate-version;
          src = nix-filter {
            root = ./.;
            include = [
              "crates"
              "examples"
              "src"
              "Cargo.toml"
              "Cargo.lock"
              "build.rs"
            ];
          };
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };
        };

      mk-dev-shell =
        { pkgs, editor }:
        pkgs.mkShell {
          inputsFrom = [ editor ];
        };

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
          editor = mk-editor { inherit pkgs rust-platform; };
          dev-shell = mk-dev-shell { inherit pkgs editor; };
        in
        {
          inherit dev-shell editor;
        };
    in
    {
      packages = nixpkgs.lib.genAttrs supported-systems (
        system:
        let
          build = mk-build { inherit nixpkgs system overlays; };
        in
        {
          default = build.editor;
          editor = build.editor;
        }
      );
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
