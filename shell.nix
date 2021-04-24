with (import (fetchTarball https://github.com/nixos/nixpkgs/archive/nixpkgs-unstable.tar.gz) { });
with pkgs;
mkShell rec {
  name = "shell";
  nativeBuildInputs = [ pkgconfig gdb ];
  buildInputs = [
    freetype
    expat
    vulkan-loader
    vulkan-tools
    wayland
    wayland-protocols
    alsaLib
    libxkbcommon
    swiftshader
    openssl
    git
  ] ++ (with xorg; [
    libX11
    libXcursor
    libXrandr
    libXi
  ]);
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath buildInputs}";
  '';
}
