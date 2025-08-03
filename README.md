# nix-stat

NixOS system health dashboard in Rust (Axum).

## What does the dashboard show?

The web interface displays live system stats: CPU, RAM, disk, network, sensors, uptime, running services, and Tailscale status.

## Quickstart (NixOS module)

1. Copy this repo to your NixOS machine (e.g. `/home/asahi/nix_stat`).
2. In your `/etc/nixos/configuration.nix`:

```nix
imports = [ /home/asahi/nix_stat/module.nix ];

services.nix-stat.enable = true;
# Optionally set a custom port:
# services.nix-stat.port = 8000;
```

3. Rebuild and start:

```sh
sudo nixos-rebuild switch
```

4. Open `http://<your-ip>:8000` in your browser.

---
- Uses vendored Rust dependencies for reproducible/offline builds.
- All HTML+JS is in `templates/index.html` (single file).
- Service runs as user `nix-stat` on all interfaces.
