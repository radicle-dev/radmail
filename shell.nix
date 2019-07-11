{ pkgs ? import <nixpkgs> {} }:
with pkgs;
pkgs.mkShell {
  buildInputs = [ pkgconfig cargo rustc ];
  # shellHook = ''export CFG_DISABLE_CROSS_TESTS=1'';
}
