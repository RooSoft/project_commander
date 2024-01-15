{
  description = "Project Commander";

  # outputs = { self, nixpkgs }: {
  #   packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;
  #   packages.x86_64-linux.default = self.packages.x86_64-linux.hello;
  # };

  inputs = { 
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };
      in 
      {
        defaultPackage = (import ./default.nix { inherit pkgs naersk'; });
      }
    );
}
