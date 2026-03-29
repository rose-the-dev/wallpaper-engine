# Premise
This is an educational project, to better learn Rust and UI design (I am terrible)

## Installation
### Nixos

Add the github as a flake input and add the module 'wallpaper-engine.homeManagerModules.wallpaper-engine' to imports / modules.
Enable the service with services.wallpaper-engine.enable = true;
This *should* add wallpaper-gui and wallpaper-runner to path.

**Currently broken:** wallpaper-gui doesn't run by default due to dependency issues (I honestly have no clue how to fix it)
but running it via "steam-run wallpaper-gui" does get it to run.
All I know is that the winit crate has issues with dynamically loading wayland client or something.

Any help is appreciated.

Example flake.nix:
```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    wallpaper-engine.url = "github:rose-the-dev/wallpaper-engine";
  };
  outputs = inputs@{ nixpkgs, home-manager, wallpaper-engine, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    nixosConfigurations."<hostname>" = nixpkgs.lib.nixosSystem {
      specialArgs = {
        inherit system;
        inherit inputs;
      };
      modules = [
        ./configuration.nix
        home-manager.nixosModules.home-manager {
          home-manager = {
            useGlobalPkgs = true;
            useUserPackages = true;
            extraSpecialArgs = { inherit inputs; };
            users.rose = {
              imports = [
                ./home.nix
                wallpaper-engine.homeManagerModules.wallpaper-engine { services.wallpaper-engine.enable = true; }
              ];
            };
          };
        }
      ];
    };
  };
}
```

### Other distros
I do have plans to create a build and install script for other distros, but right now it's nixos only.


## Current plans
### Makefile for manual install on other distros
I really don't know what I am doing when it comes to packaging stuff, even on nixos.
But that is in the works too, I need to set up an old laptop or VM to test this stuff out though.
Also I heard rust has build and install scripts but I have no clue how these work and if they can replace makefiles, I need help in that regard :/

### wallpaper-engine without linux-wallpaperengine
Basically I have plans to implement:
 - My own video player of sorts in wallpaper-engine.
 - A basic scene function, with mouse events and the ability to add effects, particles and various other stuff.
   This will be managed with a programming language (probably lua but I am open to other stuff).

### WallpaperHub
A place to be able to download and update wallpapers, this is the reason why I don't want a dependency on linux-wallpaperengine (And steam wp engine).
However I am sure most of these *could* be converted as most are simple backdrops or videos, however, I will not be creating a converter.

### Nixos module (non home-manager)
This is low priority but I will make a non home manager version in the flake, for the few people without home-manager.
