{
  description = "Wallpaper engine manager with systemd user service";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    wallpaperPkg = pkgs.callPackage ./nix/package.nix { };
  in
  {
    packages."x86_64-linux".default = wallpaperPkg;
    overlays.default = final: prev: {
      wallpaper-manager = wallpaperPkg;
    };

    devShells.default = pkgs.mkShell rec {
      #buildInputs = with pkgs; [ linux-wallpaperengine libxkbcommon libGL wayland ];
      #packages = with pkgs; [ linux-wallpaperengine ];
      shellHook = ''
        echo "Bruh"
      '';

      #PROJECT_NAME = "wallpaper-engine";
      #RUST_LOG = "debug";
      #RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      #LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
    };
    #nixosModules.wallpaper-engine = import ./nix/module.nix;
    homeManagerModules.wallpaper-engine = import ./nix/module.nix;
  };
}