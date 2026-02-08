{
  stdenv,
  mkShell,
  makeRustPlatform,

  rust-toolchain,
  nix-filter,
}:
let
  rust-platform = makeRustPlatform {
    cargo = rust-toolchain;
    rustc = rust-toolchain;
    inherit stdenv;
  };

  workspace-version =
    cargo-toml:
    let
      string = builtins.readFile cargo-toml;
      manifest = builtins.fromTOML string;
    in
    manifest.workspace.package.version;
in
rust-platform.buildRustPackage {
  pname = "aos2-save-editor";
  version = workspace-version ../Cargo.toml;

  src = nix-filter {
    root = ../.;
    include = [
      "crates"
      "examples"
      "src"
      "Cargo.toml"
      "Cargo.lock"
      "build.rs"
      "icon.ico"
    ];
  };
  cargoLock = {
    lockFile = ../Cargo.lock;
    allowBuiltinFetchGit = true;
  };
}
