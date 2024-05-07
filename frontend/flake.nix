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

        frontend = with pkgs; mkYarnPackage {
          name = pname;
          version = "0.5.0";
          src = ./.;
          nativeBuildInputs = [ 
            cargo-tauri 
            # cargoDeps 
            nodePackages.pnpm 
            cargo 
            nodejs 
            jq
          ];
          buildInputs = [ 
            # cargoDeps
          ];
          # yarnFlags = ["--frozen-lockfile"];
          
          # preBuild = ''
          #   export HOME=$(mktemp -d)
          #   mkdir -p deps
          #   mkdir -p deps/core_lib
          #   cp -r ${inputs.core.packages.${system}.rust-bin.src}/* deps/core_lib
          # '';

          # preInstall = ''
          #   ls
          # '';

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
          #   runHook preBuild

          #   # yarn --offline build --verbose
          #   ls -la deps/core_lib
          #   cargo tauri build --verbose

          #   runHook postBuild
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
          
          # meta = with lib; {
          #   description = "frontend for rquickshare";
          #   homepage = "https://github.com/Martichou/rquickshare/tree/master/frontend";
          #   license = licenses.gpl3;
          #   platforms = lib.platforms.linux; #FIXME darwin #22
          #   maintainers = with maintainers; [ hannesgith ];
          # };
          
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

        cargoDeps = with pkgs; symlinkJoin {
          name = "${pname}-cargoDeps-joined";
          paths = [
            (runCommand "${pname}-rqs_lib-src" { } ''
              mkdir -p $out
              mkdir -p $out/rqs_lib
              mkdir -p $out/core_lib
              cp -r ${inputs.core.packages.${system}.rust-bin.src }/* $out/rqs_lib
              cp -r ${inputs.core.packages.${system}.rust-bin.src }/* $out/core_lib
            '')
            (rustPlatform.importCargoLock {
              lockFile = runCommand "${pname}-srcTauri-cargo.lock" { } ''
                sed '/name = "rqs_lib"/,/^$/d' ${self}/src-tauri/Cargo.lock > $out
                sed -i '/rqs_lib/d' $out
              '';
              # lockFile = ./src-tauri/Cargo.lock;
              outputHashes = {
                "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
                "tauri-plugin-autostart-0.0.0" = "sha256-uOPFpWz715jT8zl9E6cF+tIsthqv4x9qx/z3dJKVtbw=";
              };
            })
          ];
        };

        default = with pkgs; rustPlatform.buildRustPackage {
          inherit pname;
          name = "${pname}-actual";
          src = ./.;
          buildInputs = [ ];
          nativeBuildInputs = [ pkg-config ];

          sourceRoot = "source/src-tauri";

          cargoLock = {
            lockFile = runCommand "${pname}-srcTauri-cargo.lock" { } ''
                sed '/name = "rqs_lib"/,/^$/d' ${self}/src-tauri/Cargo.lock > $out
                sed -i '/rqs_lib/d' $out
              '';
            outputHashes = {
                "mdns-sd-0.10.4" = "sha256-y8pHtG7JCJvmWCDlWuJWJDbCGOheD4PN+WmOxnakbE4=";
                "tauri-plugin-autostart-0.0.0" = "sha256-uOPFpWz715jT8zl9E6cF+tIsthqv4x9qx/z3dJKVtbw=";
              };
          };

          # copy the frontend static resources to final build directory
          # Also modify tauri.conf.json so that it expects the resources at the new location
            # cp ${./Cargo.lock} Cargo.lock
          postPatch = ''

            mkdir -p frontend-build
            cp -R ${frontend}/src frontend-build

            substituteInPlace tauri.conf.json --replace '"distDir": "../dist",' '"distDir": "frontend-build/src",'
          '';

          checkFlags = [
            # tries to mutate the parent directory
            "--skip=test_file_operation"
          ];

          postInstall = ''
            mv $out/bin/app $out/bin/rquickshare
          '';

          meta = with lib; {
            description = "frontend for rquickshare";
            homepage = "https://github.com/Martichou/rquickshare/tree/master/frontend";
            license = licenses.gpl3;
            platforms = lib.platforms.linux; #FIXME darwin #22
            maintainers = with maintainers; [ hannesgith ];
          };
        };

      });
    };
}