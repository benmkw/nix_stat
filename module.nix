{ config, lib, pkgs, ... }:
with lib;
let
  cfg = config.services.nix-stat;
  nixStatPkg = pkgs.stdenv.mkDerivation {
    pname = "nix-stat";
    version = "0.1.0";
    src = ./.;

    nativeBuildInputs = with pkgs; [ rustc cargo ];

    CARGO_HOME = ".cargo";

    buildPhase = ''
      export CARGO_HOME=$PWD/.cargo
      export RUSTC_BOOTSTRAP=1
      cargo build --release --offline
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp target/release/nix_stat $out/bin/
    '';

    meta = with pkgs.lib; {
      description = "NixOS system health dashboard in Rust (Axum)";
      homepage = "https://github.com/username/nix-stat";
      license = licenses.mit;
      platforms = platforms.linux;
      maintainers = [ ];
    };
  };
in {
  options.services.nix-stat = {
    enable = mkEnableOption "Nix Stat Dashboard";
    port = mkOption {
      type = types.port;
      default = 8000;
      description = "Port to listen on.";
    };
    logLevel = mkOption {
      type = types.enum [ "error" "warn" "info" "debug" "trace" ];
      default = "info";
      description = "Log level for the application.";
    };
    openFirewall = mkOption {
      type = types.bool;
      default = true;
      description = "Whether to automatically open the firewall port.";
    };
  };

  config = mkIf cfg.enable {
    users.users.nix-stat = {
      isSystemUser = true;
      group = "nix-stat";
      extraGroups = [ "systemd-journal" ];
    };
    users.groups.nix-stat = {};

    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];

    systemd.services.nix-stat = {
      description = "Nix Stat Dashboard";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" "tailscaled.service" ];
      serviceConfig = {
        User = "nix-stat";
        Group = "nix-stat";
        ExecStart = "${nixStatPkg}/bin/nix_stat";
        Restart = "always";
        RestartSec = "10s";
        StateDirectory = "nix-stat";
        # Security hardening
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectHome = true;
        ProtectSystem = "strict";
        ReadWritePaths = [ "/var/lib/nix-stat" ];
        Environment = [
          "PATH=${pkgs.lib.makeBinPath [
            pkgs.coreutils
            pkgs.iproute2
            pkgs.systemd
            pkgs.bash
            pkgs.tailscale
            pkgs.lm_sensors
            pkgs.procps
            pkgs.util-linux  # for ss command
            pkgs.gawk        # for text processing
          ]}"
          "RUST_LOG=${cfg.logLevel}"
          "PORT=${toString cfg.port}"
        ];
      };
    };
    environment.systemPackages = [ nixStatPkg ];
  };
}
