{ sources ? import ./sources.nix { }
, pkgs ? import sources.nixpkgs {
    overlays = [
      (import "${sources.rust-overlay}")
    ];
  }
}:

let
  gitignoreSrc = import ./gitignore.nix { inherit sources; };
  # rust-bin comes from the rust-overlay
  rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
  craneBase = import sources.crane { };
  crane = craneBase.overrideToolchain rustToolchain;
  src = gitignoreSrc ../.;

  commonArgs = {
    pname = "rust-uefi-hello-world-x86_64";
    version = "0.0.0";
    inherit src;
    cargoExtraArgs = "--target x86_64-unknown-uefi";
  };
  # Downloaded and compiled dependencies.
  cargoArtifacts = crane.buildDepsOnly (commonArgs // {
    doCheck = false;
  });
  cargoTest = crane.cargoTest (commonArgs // {
    inherit cargoArtifacts;
    cargoExtraArgs = "--lib --target x86_64-unknown-linux-gnu";
  });
  cargoPackage = crane.buildPackage (commonArgs // {
    # Don't execute tests here. There is a dedicated Nix attribute.
    doCheck = false;
    inherit cargoArtifacts;
  });

  #runInQemu = pkgs.writeBashScriptBin "" {}
in
{
  efi-app = {
    artifacts = cargoArtifacts;
    app = cargoPackage;
    test = cargoTest;
  };
}
