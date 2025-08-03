{ nixpkgs ? <nixpkgs> }:

let
  system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system; };
  
  # Import NixOS evaluation system
  eval-config = import (nixpkgs + "/nixos/lib/eval-config.nix");
in
eval-config {
  inherit system;
  modules = [
    ./module.nix
    ({ config, ... }: {
      services.nix-stat.enable = true;
    })
  ];
}
