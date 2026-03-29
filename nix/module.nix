{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.wallpaper-engine;
  wallpaper-engine = pkgs.callPackage ./package.nix { };
in
{
  options.services.wallpaper-engine = {
    enable = mkEnableOption "Enable the wallpaper-runner user service";

    package = mkOption {
      type = types.package;
      default = wallpaper-engine;
      description = "The wallpaper-engine package to use.";
    };

    serviceRestartMode = mkOption {
      type = types.enum [ "no" "always" "on-success" "on-failure" "on-abnormal" "on-abort" "on-watchdog" ];
      default = "always";
      description = "Define the restart mode for the service, \"no\", \"always\", \"on-success\", \"on-failure\", \"on-abnormal\", \"on-abort\", \"on-watchdog\".";
    };
  };

  config = mkIf cfg.enable {
    systemd.user.services.wallpaper-engine = {
      description = "Wallpaper-engine Service.";
      after = [ "graphical-session.target" ];
      serviceConfig = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/wallpaper-engine";
        Restart = cfg.serviceRestartMode;
        RestartSec = 5;
      };
      wantedBy = [ "graphical-session.target" ];
    };
    environment.systemPackages = [ cfg.package ];
  };
}