{
  description = "rquickshare-core flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = inputs@{ nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      # definitely not tested, but i like it exposed, who knows
      systems = lib.systems.flakeExposed;
      forAllSystems = lib.genAttrs systems;
      spkgs = system : nixpkgs.legacyPackages.${system}.pkgs;
    in
    {
      packages = forAllSystems (system: with spkgs system; {
        rcore = rustPlatform.buildRustPackage {
          name = "rquickshare-core";
          src = ./.;
          nativeBuildInputs = [  ];
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
            };
          };
        };
        default = rquickshare;
      });
    };
}
