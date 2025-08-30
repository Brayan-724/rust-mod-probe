{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    fenix-flake.url = "github:nix-community/fenix";
    fenix-flake.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {nixpkgs,fenix-flake, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    fenix = fenix-flake.packages.${system};
    rust-toolchain = fenix.fromToolchainFile {
      file = ./rust-toolchain.toml;
      sha256 = "sha256-VW/GbYzsXuN/9RFwVWtqVIC6w9YwtTuWsCRelwD1Npw=";
    };

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
        rust-toolchain

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

      RUST_BACKTRACE = "1";
    };
  };
}
