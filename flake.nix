{
  inputs = {
    naersk.url = "github:nix-community/naersk";
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = {self, nixpkgs, naersk, utils, ... }:
    utils.lib.eachDefaultSystem (system: 
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
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
            ];
            buildInputs = [
              cargo
              rustc
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
