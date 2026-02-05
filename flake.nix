{
  description = "Wallpaper engine gui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # devShells.default is the development environment
        # It's activated with 'nix develop' or automatically via direnv
        # Documentation: https://nixos.wiki/wiki/Development_environment_with_nix-shell
        devShells.default = pkgs.mkShell {
              # Packages to include in the shell environment
              buildInputs = with pkgs; [
                mpv
                pkg-config
              ];

              shellHook = ''
                echo "Bruh"
              '';

              PROJECT_NAME = "wallpaper-engine";
            };
      });
}