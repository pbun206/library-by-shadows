{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  strictDeps = true;
  nativeBuildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.rust-analyzer
    pkgs.pkg-config
  ];
  buildInputs = [
    pkgs.openssl
  ];
}
