{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath = with pkgs; lib.makeLibraryPath [
          mesa
          xorg.libX11
          xorg.libXcursor
          xorg.libXxf86vm
          xorg.libXi
          xorg.libXrandr
         ];    
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            rust-bin.stable.latest.default

            # mesa
            # xorg.libX11
            # xorg.libXcursor
            # xorg.libXxf86vm
            # xorg.libXi
            # xorg.libXrandr
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${libPath}
          '';
        };
      }
    );
}
