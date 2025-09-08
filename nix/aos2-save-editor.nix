{
  stdenv,
  mkShell,
  nix-filter,
  makeRustPlatform,

  rust-toolchain,
}:
let
  rust-platform = makeRustPlatform {
    cargo = rust-toolchain;
    rustc = rust-toolchain;
  };

  workspace-version =
    cargo-toml:
    let
      string = builtins.readFile cargo-toml;
      manifest = builtins.fromTOML string;
    in
    manifest.workspace.package.version;

  aos2-save-editor = rust-platform.buildRustPackage {
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
  };

  dev-shell = mkShell {
    inputsFrom = [ aos2-save-editor ];
  };
in
{
  inherit aos2-save-editor dev-shell;
}
