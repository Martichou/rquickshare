{
  description = "rquickshare frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    core.url = "./../core_lib";
  };
  
  outputs = inputs@{ self, nixpkgs,... }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ]; # ++ [ "x86_64-darwin" "aarch64-darwin" ]; #FIXME darwin #22
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pname = "rquickshare-frontend";
      pkgs-s = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system: let pkgs = pkgs-s.${system}; in rec {

        default = with pkgs; mkYarnPackage {
          name = pname;
          version = "0.5.0";
          src = ./.;
          nativeBuildInputs = [ cargo-tauri cargoDeps nodePackages.pnpm cargo nodejs jq];
          yarnFlags = ["--frozen-lockfile"];
          
          # buildInputs = [ gtk3 wayland xorg.libxcb.dev ];
          # preBuild = ''
          #   export HOME=$(mktemp -d)
          #   pnpm config set store-dir ${pnpmDeps}
          #   chmod +w ..
          #   pnpm install --offline --frozen-lockfile --no-optional --ignore-script
          #   ls -la node_modules/.pnpm/@tauri-apps+cli@1.5.7/node_modules/@tauri-apps/cli/
          #   # pnpm rebuild
          #   echo "pnpm install done, building cargo"
          #   # Use cargo-tauri from nixpkgs instead of pnpm tauri from npm
          #   cargo tauri build -v
          # '';

          # preInstall = ''
          #   ls
          # '';

          # pkgConfig = {
          #   core_lib = {
              
          #   };
          # };

          # preBuild = ''
          #   export HOME=$(mktemp -d)
          #   pnpm config set store-dir $out
          #   pnpm install --frozen-lockfile --no-optional --ignore-script
          #   # pnpm rebuild
          #   echo "pnpm install done, building cargo"
          #   # Use cargo-tauri from nixpkgs instead of pnpm tauri from npm
          #   cargo tauri build -v
          # '';

          pkgConfig.core_lib = inputs.core.packages.${system}.node-module;
          # packageJSON = patchedPackageJSON ;#src + "/nix-modded-package.json";
          packageJSON = pkgs.runCommand "${pname}-package.json" { } ''
            ${jq}/bin/jq 'del(.dependencies["@martichou/core_lib"])' ${self}/package.json > $out
            cat $out
          '';

          # workspaceDependencies = [
          #   (inputs.core.packages.${system}.node-module)
          #   # {
          #   #   name = "core_lib";
          #   #   path = "../core_lib";
          #   # }
          # ];

          # yarnPreBuild = ''
          #   # mkdir -p /build/deps
          #   # cp -r ${inputs.core.packages.${system}.node-module}/libexec/@martichou/core_lib/deps/@martichou/core_lib /build/deps
          #   # ls -la /build/deps
          # '';

          # configurePhase = ''
          #   ln -s $node_modules node_modules
          #   ls -la $node_modules
          #   echo "${nodejs}/bin/node $node_modules/vite/bin/vite.js" >> vite
          #   chmod +x vite
          #   export PATH=$PWD:$PATH
          #   cp -r ${inputs.core.packages.${system}.node-module}/libexec/@martichou/core_lib/deps/@martichou/ /build/deps
          #   ls -la /build/deps
          # '';

          # buildPhase = ''
          #   # yarn --offline build --verbose
          # '';

          # offlineCache = symlinkJoin { 
          #   name = "${pname}-offlineCache";
          #   paths = [
          #     (yarn2nix-moretea.importOfflineCache (yarn2nix-moretea.mkYarnNix { yarnLock = ./yarn.lock; }))
          #     (yarn2nix-moretea.importOfflineCache (yarn2nix-moretea.mkYarnNix { yarnLock = ../core_lib/yarn.lock; }))
          #     (inputs.core.packages.${system}.node-module + "/tarballs")
          #     (inputs.core.packages.${system}.offline-cache)
          #   ];
          # };

          
          meta = with lib; {
            description = "frontend for rquickshare";
            homepage = "https://github.com/Martichou/rquickshare/tree/master/frontend";
            license = licenses.gpl3;
            platforms = lib.platforms.linux; #FIXME darwin #22
            maintainers = with maintainers; [ hannesgith ];
          };
          # ESBUILD_BINARY_PATH = "${lib.getExe (esbuild.override {
          #   buildGoModule = args: buildGoModule (args // rec {
          #     version = "0.19.8";
          #     src = fetchFromGitHub {
          #       owner = "evanw";
          #       repo = "esbuild";
          #       rev = "v${version}";
          #       hash = "sha256-f13YbgHFQk71g7twwQ2nSOGA0RG0YYM01opv6txRMuw=";
          #     };
          #   });
          # })}";
        };

        # pnpmDeps = with pkgs; stdenvNoCC.mkDerivation {
        #   name = "${pname}-pnpm-deps";
        #   src = ./.;
        #   nativeBuildInputs = [ jq moreutils nodePackages.pnpm cacert ];
        #   installPhase = ''
        #     export HOME=$(mktemp -d)
        #     pnpm config set store-dir $out
        #     # use --ignore-script and --no-optional to avoid downloading binaries
        #     # use --frozen-lockfile to avoid checking git deps
        #     pnpm install --frozen-lockfile --no-optional --ignore-script

        #     # Remove timestamp and sort the json files
        #     rm -rf $out/v3/tmp
        #     for f in $(find $out -name "*.json"); do
        #       sed -i -E -e 's/"checkedAt":[0-9]+,//g' $f
        #       jq --sort-keys . $f | sponge $f
        #     done
        #   '';

        #   # dontFixup = true;
        #   outputHashMode = "recursive";
        #   outputHash = "sha256-jC0GZOx5PjIXY3ayS9M2qFdK4F/p0wzdM/NPdPShTqY=";
        # };

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