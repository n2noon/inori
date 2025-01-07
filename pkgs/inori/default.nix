{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "inori";
  version = "0.2.2";
  src = ../..;

  cargoLock.lockFile = ../../Cargo.lock;

  meta = with lib; {
    description = "Client for the Music Player Daemon (MPD)";
    homepage = "https://github.com/eshrh/inori";
    license = licenses.gpl3Only;
    platforms = platforms.all;
    mainProgram = "inori";
    maintainers = with maintainers; [ stephen-huan ];
  };
}
