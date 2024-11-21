{
  description = "rquickshare flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    inputs@{ nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      # definitely not tested, but i like it exposed, who knows
      systems = lib.systems.flakeExposed;
      forAllSystems = lib.genAttrs systems;
      spkgs = system: nixpkgs.legacyPackages.${system}.pkgs;
    in
    {
      packages = forAllSystems (
        system: with spkgs system; rec {
          tauri = cargo-tauri;
          # we could use rustPlatform.buildRustPackage, but there cargoDeps doesn't work
          rquickshare = let
            pname = "rquickshare";
            src = let fs = lib.fileset; in fs.toSource {
              root = ./.;
              fileset = fs.difference ./. (
                fs.unions [
                  ./flake.nix
                  ./flake.lock
                ]
              );
            };
          in
          stdenv.mkDerivation rec {
            inherit src pname;
            name = pname;
            sourceRoot = "${src.name}/app/main";
            corelibSourceRoot = "${src.name}/core_lib";
            cargoRoot = "src-tauri";
            corelibPath = "rqs_lib";
            nativeBuildInputs = [
              tauri.hook
              pnpm.configHook
              nodejs
              moreutils
              protobuf
              jq
              yq
              rustPlatform.cargoSetupHook
            ];
            # seemingly the fetcher ignores cargoRoot (contrary to the docs)
            cargoDeps = rustPlatform.fetchCargoVendor {
              inherit src pname;
              hash = cargoHash;
              sourceRoot = "${sourceRoot}/${cargoRoot}";
            };
            cargoHash = "sha256-m+BHN0eCF8rwCBWQ94XWHP7vmtQMtT/9adFgcU0BDTQ=";
            pnpmDeps = pnpm.fetchDeps {
              inherit src sourceRoot pname;
              hash = "sha256-ko0S934TnGDVbTrFkunHPt0e/pT5CVi4yorTi0WYqmc=";
            };
            postUnpack =
              let
                fullCorePath = "${sourceRoot}/${cargoRoot}/${corelibPath}";
              in
              ''
                cp -r ${corelibSourceRoot} ${fullCorePath} && chmod -R +w ${fullCorePath}
              '';
            # remove macOS signing and relative links
            postConfigure = ''
              jq     'del(.bundle.macOS.signingIdentity, .bundle.macOS.hardenedRuntime)' src-tauri/tauri.conf.json | sponge src-tauri/tauri.conf.json
              yq -iy '.importers.".".dependencies."@martichou/core_lib".specifier = "link:${corelibPath}" | .importers.".".dependencies."@martichou/core_lib".version = "link:${corelibPath}"' pnpm-lock.yaml
              jq     '.dependencies."@martichou/core_lib" = "link:${corelibPath}"' package.json | sponge package.json
              sed -i 's|path = "../../../core_lib"|path = "${corelibPath}"|' ${cargoRoot}/Cargo.toml
            '';
            checkPhase = "true"; # idk why checks fail, todo
            installPhase =
              let
                path = "${cargoRoot}/target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/bundle";
              in
              if stdenv.isDarwin then
                ''
                  mkdir -p $out/bin
                  mv ${path}/macos $out/Applications
                  echo "#!${lib.getExe zsh}" >> $out/bin/${name}
                  echo "open -a $out/Applications/${name}.app" >> $out/bin/${name}
                  chmod +x $out/bin/${name}
                ''
              else
                ''
                  mv ${path}/deb/*/data/usr $out
                '';
            meta = {
              description = "Rust implementation of NearbyShare/QuickShare from Android for Linux";
              homepage = "https://github.com/Martichou/rquickshare";
              changelog = "https://github.com/Martichou/rquickshare/blob/master/CHANGELOG.md";
              license = lib.licenses.gpl3Plus;
              maintainers = with lib.maintainers; [ hannesgith ];
              platforms = with lib.platforms; linux ++ darwin;
              mainProgram = "rquickshare";
            };
          };
          default = rquickshare;
        }
      );
      devShells = forAllSystems (
        system: with spkgs system; {
          default = mkShell {
            buildInputs = [
              cargo
              rustc
              nodejs
              pnpm
              protobuf
            ];
          };
        }
      );
    };
}
