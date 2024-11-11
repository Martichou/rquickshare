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
      packages = forAllSystems (system: with spkgs system; rec {
        rqscore = rustPlatform.buildRustPackage rec {
          name = "rquickshare-core";
          src = ./.;
          nativeBuildInputs = [ protobuf ];
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
            };
          };
          installPhase = ''
            mkdir -p $out/bin
            cp -r package.json $out/
            cp -r esm $out/
            cp -r dist $out/
            cp -r target $out/
            cp -r target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/core_bin $out/bin/${name}
          '';
        };
        default = rqscore;
      });
    };
}
