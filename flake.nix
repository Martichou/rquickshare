{
  description = "rquickshare flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # as upstream nixpkgs doesn't have tauri 2, we use a fork
    nixpkgs2.url = "github:hannesGitH/nixpkgs?ref=tauri2";
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
        remote2 = inputs.nixpkgs2.legacyPackages.${system};
        tauri = remote2.pkgs.cargo-tauri;
        rquickshare = rustPlatform.buildRustPackage rec {
          name = "rquickshare";
          src = ./app/main;
          nativeBuildInputs = [ tauri.hook pnpm.configHook nodejs ];
          cargoRoot = "src-tauri";
          cargoLock = {
            lockFile = app/main/src-tauri/Cargo.lock;
            outputHashes = {
              "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
            };
          };
          pnpmDeps = pnpm.fetchDeps {
            inherit src;
            pname = name;
            hash = "";
          };
          # remove macOS signing and relative links
          postConfigure = ''
            ${jq}/bin/jq -i 'del(.bundle.macOS.signingIdentity, .bundle.macOS.hardenedRuntime)' src-tauri/tauri.conf.json
            ${yq}/bin/yq -i '.importers.".".dependencies."@martichou/core_lib".specifier = "link:${corelib}" | .importers.".".dependencies."@martichou/core_lib".version = "link:${corelib}"' pnpm-lock.yaml
          '';
          # checkPhase = "true"; # idk why checks fail, todo
          # installPhase = let path = "${cargoRoot}/target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/bundle"; in 
          # if stdenv.isDarwin then ''
          #   mkdir -p $out/bin
          #   mv ${path}/macos $out/Applications
          #   echo "#!${zsh}/bin/zsh" >> $out/bin/${name}
          #   echo "open -a $out/Applications/${name}.app" >> $out/bin/${name}
          #   chmod +x $out/bin/${name}
          # '' else ''
          #   mv ${path}/deb/*/data/usr $out
          # '';
        };
        default = rquickshare;
      });
      devShells = forAllSystems (system: with spkgs system; {
        default = mkShell {
          buildInputs = [ cargo rustc nodejs ];
        };
      });
    };
}
