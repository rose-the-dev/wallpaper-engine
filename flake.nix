{
  description = "Wallpaper engine manager with systemd user service";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    wallpaper-engine = pkgs.callPackage ./nix/package.nix { };
  in
  {
    packages."x86_64-linux".default = wallpaper-engine;
    overlays.default = final: prev: {
      wallpaper-engine = wallpaper-engine;
    };
    #nixosModules.wallpaper-engine = import ./nix/module.nix;
    homeManagerModules.wallpaper-engine = import ./nix/hm-module.nix;
  };
}