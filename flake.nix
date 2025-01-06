{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    libraries = with pkgs; [
        glfw3-minecraft

        libGL
        libGLX

        alsa-lib
        libjack2
        libpulseaudio
        pipewire

        xorg.libX11
        xorg.libXcursor
        xorg.libXext
        xorg.libXrandr
        xorg.libXxf86vm

        udev
    ];
  in {
    # nix develop
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        jdk21
        jdt-language-server

        openal

        mesa-demos
        xorg.xrandr
      ] ++ libraries;

      buildInputsNative = with pkgs; [
        pkg-config
      ];

      LD_LIBRARY_PATH = "${pkgs.addDriverRunpath.driverLink}/lib:${pkgs.lib.makeLibraryPath libraries}";
    };
  };
}
