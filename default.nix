{
  lib,
  rustPlatform,
  copyDesktopItems,
  makeDesktopItem,
}:
rustPlatform.buildRustPackage {
  pname = "bouncer";
  version = "0.1.0";

  cargoHash = "sha256-ilLu+V1fQaBkdFf/D7naopg/tbGLX5/pM1kpDk/jpIU=";

  src = lib.cleanSource ./.;

  nativeBuildInputs = [copyDesktopItems];

  desktopItems = [
    (makeDesktopItem {
      name = "bouncer";
      desktopName = "Bouncer";
      comment = "Screen links before opening in a browser";
      exec = "bouncer %u";
      terminal = true;
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
