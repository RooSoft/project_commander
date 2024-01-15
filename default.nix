{ pkgs, nixpkgs, flake-utils, naersk' }:

  naersk'.buildPackage {
    src = pkgs.fetchFromGitHub {
      owner = "roosoft";
      repo = "project_commander";
      rev = "58c81f52d922e4b4c972544637befe1eafff0ecd";
      sha256 = "sha256-2PoOPc2zIRA6jPESDfmza33n9AaOlO9luoRdozBlFqI=";
    };

    nativeBuildInputs = with pkgs; [ pkg-config ];
    buildInputs = with pkgs; [ openssl ];
  }
