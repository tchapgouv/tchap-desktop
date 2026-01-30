{
  description = "Tchap linux";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
    supportedSystems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
    forAllSupportedSystems = fn: 
    let
      genAttrs = keys: fn: builtins.foldl' (acc: item: acc // { ${item} = fn item; }) { } keys;
    in
      genAttrs supportedSystems (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in fn system pkgs
    );
  in
  {
    packages = forAllSupportedSystems (system: pkgs: {
      tchap-desktop = pkgs.callPackage ./nix/package.nix { };
      default = self.packages.${system}.tchap-desktop;
    });
  };
}
