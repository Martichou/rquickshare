{
  description = "rquickshare core lib";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = inputs@{ self, nixpkgs,... }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ]; # ++ [ "x86_64-darwin" "aarch64-darwin" ]; #FIXME darwin #22
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs-s = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system: let pkgs = pkgs-s.${system}; in rec {
        offline-cache = with pkgs; (yarn2nix-moretea.importOfflineCache (yarn2nix-moretea.mkYarnNix { yarnLock = ./yarn.lock; }));
        rust-bin = with pkgs; rustPlatform.buildRustPackage {
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ dbus protobuf ];
          PROTOC = "${protobuf}/bin/protoc";
          pname = "rqs_lib";
          version = "1.0.9";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
            };
          };
          meta = with lib; {
            description = "Core Lib for rquickshare";
            homepage = "https://github.com/Martichou/rquickshare/tree/master/core_lib";
            license = licenses.gpl3;
            maintainers = with maintainers; [ hannesgith ];
          };
        };
        node-module = with pkgs; mkYarnPackage {
          pname = "rqs_lib";
          version = "1.0.9";
          src = ./.;
          offlineCache = offline-cache;
          # buildStep = ''
          #   yarn --offline build
          # '';
          meta = with lib; {
            description = "Core Lib for rquickshare";
            homepage = "https://github.com/Martichou/rquickshare/tree/master/core_lib";
            license = licenses.gpl3;
            maintainers = with maintainers; [ hannesgith ];
          };
        };
        default = node-module;
      });
    };
}