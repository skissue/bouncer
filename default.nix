{
  lib,
  rustPlatform,
  copyDesktopItems,
  makeDesktopItem,
  pkg-config,
  wayland,
  libxkbcommon,
  libGL,
}:
rustPlatform.buildRustPackage {
  pname = "bouncer";
  version = "0.1.0";

  cargoLock.lockFile = ./Cargo.lock;

  src = lib.cleanSource ./.;

  nativeBuildInputs = [copyDesktopItems pkg-config];

  postFixup = ''
    patchelf --add-rpath ${lib.makeLibraryPath [wayland libxkbcommon libGL]} $out/bin/bouncer
  '';

  desktopItems = [
    (makeDesktopItem {
      name = "bouncer";
      desktopName = "Bouncer";
      comment = "Screen links before opening in a browser";
      exec = "bouncer %u";
      terminal = false;
      mimeTypes = ["x-scheme-handler/http" "x-scheme-handler/https"];
      categories = ["Network" "WebBrowser"];
    })
  ];

  meta = {
    description = "Bouncer";
    license = lib.licenses.mit;
    mainProgram = "bouncer";
    platforms = lib.platforms.linux;
  };
}
