{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [
    rust-analyzer
    rustfmt
    cargo
    rustc
    pkg-config

    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi

    wayland
    wayland-protocols
    libxkbcommon
  ];
}
