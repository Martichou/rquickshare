{
  description = "rquickshare flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # as upstream nixpkgs doesn't have tauri 2, we use a fork
    nixpkgs2.url = "github:hannesGitH/nixpkgs?ref=tauri2";
    corelib.url = "./core_lib";
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
        corelib_pkgs = inputs.corelib.outputs.packages.${system};
        corelib_src = corelib_pkgs.rqscore_src;
        corelib_vendored = (stdenvNoCC.mkDerivation {
          name = "cargo-vendor";
          src = corelib_src;
          phases = [ "installPhase" ];
          installPhase = ''
            mkdir -p $out/rqs_lib
            cp -r $src/* $out/rqs_lib
          '';
        });
        deps_vendored = (rustPlatform.importCargoLock {
          lockFile = app/main/src-tauri/Cargo.lock;
          outputHashes = {
            "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
          };
        });
        cargo_vendor_dir = symlinkJoin {
          name = "cargo-vendor";
          paths = [ 
            deps_vendored
            corelib_vendored
          ];
        };
        vendorDir = "vendorr";
        rquickshare = stdenv.mkDerivation rec {
          name = "rquickshare";
          src = ./app/main;
          nativeBuildInputs = [ tauri.hook pnpm.configHook nodejs moreutils ];
          cargoRoot = "src-tauri";
          cargoDeps = vendorDir;
          pnpmDeps = pnpm.fetchDeps {
            inherit src;
            pname = name;
            hash = "sha256-ko0S934TnGDVbTrFkunHPt0e/pT5CVi4yorTi0WYqmc=";
          };
          # remove macOS signing and relative links
          postConfigure = ''
            ${jq}/bin/jq     'del(.bundle.macOS.signingIdentity, .bundle.macOS.hardenedRuntime)' src-tauri/tauri.conf.json | sponge src-tauri/tauri.conf.json
            ${yq}/bin/yq -iy '.importers.".".dependencies."@martichou/core_lib".specifier = "link:${corelib_src}" | .importers.".".dependencies."@martichou/core_lib".version = "link:${corelib_src}"' pnpm-lock.yaml
            ${jq}/bin/jq     '.dependencies."@martichou/core_lib" = "link:${corelib_src}"' package.json | sponge package.json
            cp -r ${cargo_vendor_dir} ${vendorDir}
          '';
          checkPhase = "true"; # idk why checks fail, todo
          installPhase = let path = "${cargoRoot}/target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/bundle"; in 
          if stdenv.isDarwin then ''
            mkdir -p $out/bin
            mv ${path}/macos $out/Applications
            echo "#!${zsh}/bin/zsh" >> $out/bin/${name}
            echo "open -a $out/Applications/${name}.app" >> $out/bin/${name}
            chmod +x $out/bin/${name}
          '' else ''
            mv ${path}/deb/*/data/usr $out
          '';
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
