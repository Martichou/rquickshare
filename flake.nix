{
  description = "rquickshare: quickshare for linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    core.url = "./core_lib";
    frontend.url = "./frontend";
  };
  
  outputs = inputs@{ self, nixpkgs, ... }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ]; # ++ [ "x86_64-darwin" "aarch64-darwin" ]; #FIXME darwin #22
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs-s = forAllSystems (system: import nixpkgs { inherit system; });
    in rec {
      packages = forAllSystems (system: let pkgs = pkgs-s.${system}; in rec {
        core = inputs.core.packages.${system}.default;
        web = inputs.frontend.packages.${system}.default;
        default = pkgs.stdenv.mkDerivation {
          
        };
      });
    };
}