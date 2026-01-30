{
  description = "Wallpaper engine gui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs = {self, nixpkgs}:
  let
    #pkgs = nixpkgs.legacyPackages."x86_64-linux";
    #pname = "rosethedev";
    #node = pkgs.nodejs_22;
    #nodeDeps = import ./node-packages.nix { inherit pkgs; };
  in {
    #packages."x86_64-linux".default = pkgs.callPackage ./default.nix {};

    packages = eachSystem (system: {
      default = self.packages.${system}.wallpaper-gui;
      #wallpaper-gui = callPackage ./default.nix {};
      # Funky
    });
  };
}