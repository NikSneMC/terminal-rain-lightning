{
  lib,
  rustPlatform,
  ncurses,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  pname = "terminal-rain-lightning";
  version = "0.1.0";
  src = ../.;
  cargoHash = "sha256-v2lqCfiekcn8D8w4b6lJLvCqNglW/e8Z2r0XCryfU5Y=";
  buildInputs = [ncurses];
  nativeBuildInputs = [pkg-config];
  useFetchCargoVendor = true;

  meta = with lib; {
    description = ''Terminal-based ASCII rain and lightning animation'';
    homepage = "https://github.com/NikSneMC/terminal-rain-lightning";
    mainProgram = "terminal-rain-lightning";
    license = licenses.mit;
  };
}

