{ pkgs, naersk' }:
naersk'.buildPackage {
  src = pkgs.fetchFromGitHub {
    owner = "roosoft";
    repo = "project_commander";
    rev = "98de0a49a5a9d01258f90f5e1791af11327a9260";
    sha256 = "sha256-OexaK8tG9BM4Y/ROGXSAp7XKJctLyEQzKCaHDvn/+aM=";
  };

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ openssl ];
}
