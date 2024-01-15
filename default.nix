{ pkgs, nixpkgs, flake-utils, naersk' }:

# pkgs.stdenv.mkDerivation {
#   name = "Project Commander";  
#   src = ./.;
#
#   buildInputs = [ pkgs.rustc pkgs.cargo ];
#
#   buildPhase = ''
#     cargo build
#   '';
#
#   installPhase = ''
#     mkdir -p $out/bin
#     cp $src/target/debug/main $out/bin
#   '';
# }
naersk'.buildPackage {
  src = ./.;

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ openssl ];
}
