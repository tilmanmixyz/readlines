{
  inputs = {
    naersk.url = "github:nix-community/naersk";
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = {self, nixpkgs, naersk, utils, fenix, ... }:
    utils.lib.eachDefaultSystem (system: 
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          dir = ./.;
          sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
        };
        naersk-lib = naersk.lib.${system}.override {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
      in
        {
          defaultPackage = naersk-lib.buildPackage {
            src = ./.;
            doCheck = true;
            pname = "readlines";
            version = "0.0.1";
            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
            ];
            buildInputs = with pkgs; [
              xorg.libxcb
            ];
            postInstall = ''
            wrapProgram "$out/bin/readlines" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
          };
          defaultApp = utils.lib.mkApp {
            drv = self.defaultPackage.${system};
          };
          devShell = with pkgs; mkShell {
            nativeBuildInputs = [
              pkg-config
              rustToolchain
            ];
            buildInputs = [
              rustPackages.clippy
              rustfmt
              tokei

              openssl
              xorg.libxcb
            ];
            LD_LIBRARY_PATH = libPath;
          };
        });
}
