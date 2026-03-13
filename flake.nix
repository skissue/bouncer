{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = f:
      nixpkgs.lib.genAttrs ["x86_64-linux" "aarch64-linux"] (system: f nixpkgs.legacyPackages.${system});
  in {
    overlays.default = final: prev: {
      bouncer = final.callPackage ./default.nix {};
    };

    packages = forAllSystems (pkgs: rec {
      bouncer = pkgs.callPackage ./default.nix {};
      default = bouncer;
    });
  };
}
