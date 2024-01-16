{ pkgs, naersk' }:
naersk'.buildPackage {
  src = pkgs.fetchFromGitHub {
    owner = "roosoft";
    repo = "project_commander";
    rev = "3cc282755f23c5722065dd98b2f94bcf13d61bcc";
    sha256 = "sha256-eh1MCcR18Fn0lWMgjVzbflJsjDaxNheuVdhsNoZOdUs=";
  };

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ openssl ];
}
