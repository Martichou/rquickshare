{
  description = "rquickshare frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = inputs@{ self, nixpkgs,... }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ]; # ++ [ "x86_64-darwin" "aarch64-darwin" ]; #FIXME darwin #22
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pname = "rquickshare-frontend";
      pkgs-s = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system: let pkgs = pkgs-s.${system}; in rec {

        default = with pkgs; stdenv.mkDerivation {
          name = pname;
          src = ./.;
          nativeBuildInputs = [ cargo-tauri pnpmDeps cargoDeps nodePackages.pnpm cargo vite ];
          buildInputs = [ gtk3 wayland xorg.libxcb.dev ];
          preBuild = ''
            export HOME=$(mktemp -d)
            pnpm config set store-dir ${pnpmDeps}
            chmod +w ..
            pnpm install --offline --frozen-lockfile --no-optional --ignore-script
            ls -la node_modules/.pnpm/@tauri-apps+cli@1.5.7/node_modules/@tauri-apps/cli/
            # pnpm rebuild
            echo "pnpm install done, building cargo"
            # Use cargo-tauri from nixpkgs instead of pnpm tauri from npm
            cargo tauri build -v
          '';

          preInstall = ''
            ls
          '';
          
          meta = with lib; {
            description = "frontend for rquickshare";
            homepage = "https://github.com/Martichou/rquickshare/tree/master/frontend";
            license = licenses.gpl3;
            platforms = lib.platforms.linux; #FIXME darwin #22
            maintainers = with maintainers; [ hannesgith ];
          };
          ESBUILD_BINARY_PATH = "${lib.getExe (esbuild.override {
            buildGoModule = args: buildGoModule (args // rec {
              version = "0.19.8";
              src = fetchFromGitHub {
                owner = "evanw";
                repo = "esbuild";
                rev = "v${version}";
                hash = "sha256-f13YbgHFQk71g7twwQ2nSOGA0RG0YYM01opv6txRMuw=";
              };
            });
          })}";
        };

        pnpmDeps = with pkgs; stdenvNoCC.mkDerivation {
          name = "${pname}-pnpm-deps";
          src = ./.;
          nativeBuildInputs = [ jq moreutils nodePackages.pnpm cacert ];
          installPhase = ''
            export HOME=$(mktemp -d)
            pnpm config set store-dir $out
            # use --ignore-script and --no-optional to avoid downloading binaries
            # use --frozen-lockfile to avoid checking git deps
            pnpm install --frozen-lockfile --no-optional --ignore-script

            # Remove timestamp and sort the json files
            rm -rf $out/v3/tmp
            for f in $(find $out -name "*.json"); do
              sed -i -E -e 's/"checkedAt":[0-9]+,//g' $f
              jq --sort-keys . $f | sponge $f
            done
          '';

          # dontFixup = true;
          outputHashMode = "recursive";
          outputHash = "sha256-jC0GZOx5PjIXY3ayS9M2qFdK4F/p0wzdM/NPdPShTqY=";
        };

        # yarnDeps = with pkgs; 

        cargoDeps = with pkgs; rustPlatform.importCargoLock {
          lockFile = ./src-tauri/Cargo.lock;
          outputHashes = {
            "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
            "tauri-plugin-autostart-0.0.0" = "sha256-uOPFpWz715jT8zl9E6cF+tIsthqv4x9qx/z3dJKVtbw=";
          };
        };
      });
    };
}