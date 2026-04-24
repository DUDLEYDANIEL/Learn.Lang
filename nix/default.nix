let 
 pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05") { config = {}; overlays=[]; };
in 
{
    hello = pkgs.callPackage ./hello.nix { };
    icat = pkgs.callPackage ./icat.nix { };
}

