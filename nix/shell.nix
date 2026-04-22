let 
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05"){ config = {}; overlays = []; };
in 

pkgs.mkShellNoCC {
    # importing packages with compiler use
    packages = with pkgs; [
      cowsay
      lolcat
    ];

    # environment variables 
    GREETING = "Hello, Nix!";
    
    # helps to do an startup setup 
    shellHook = ''
      echo $GREETING | cowsay | lolcat
    '';
  }
