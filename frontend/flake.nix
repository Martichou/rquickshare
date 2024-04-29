{
  description = "rquickshare frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = inputs@{ self, nixpkgs,... }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ]; # ++ [ "x86_64-darwin" "aarch64-darwin" ]; #FIXME darwin #22
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs-s = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system: let pkgs = pkgs-s.${system}; in {
        default = with pkgs; rustPlatform.buildRustPackage {
          pname = "rquickshare-frontend";
          version = "0.0.0";
          src = ./.;
          cargoHash = lib.fakeHash;
          meta = {
          description = "frontend for rquickshare";
            homepage = "https://github.com/Martichou/rquickshare/tree/master/frontend";
            license = licenses.gplv3;
            maintainers = with maintainers; [ hannesgith ];
          };
        };
      });
    };
}