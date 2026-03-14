{
  lib,
  rustPlatform,
  copyDesktopItems,
  makeDesktopItem,
  gui ? false,
  pkg-config,
  wayland,
  libxkbcommon,
  libGL,
}:
rustPlatform.buildRustPackage {
  pname = "bouncer${lib.optionalString gui "-gui"}";
  version = "0.1.0";

  cargoLock.lockFile = ./Cargo.lock;

  src = lib.cleanSource ./.;

  buildNoDefaultFeatures = true;
  buildFeatures = [
    (
      if gui
      then "gui"
      else "tui"
    )
  ];

  nativeBuildInputs = [copyDesktopItems] ++ lib.optionals gui [pkg-config];

  postFixup = lib.optionalString gui ''
    patchelf --add-rpath ${lib.makeLibraryPath [wayland libxkbcommon libGL]} $out/bin/bouncer
  '';

  desktopItems = [
    (makeDesktopItem {
      name = "bouncer";
      desktopName = "Bouncer";
      comment = "Screen links before opening in a browser";
      exec = "bouncer %u";
      terminal = !gui;
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
