{ pkgs }:

let
  commonPackages = with pkgs; [
    cargo
    cargo-audit
    cargo-deny
    cargo-nextest
    clippy
    git
    jq
    just
    nixfmt
    openssl
    pkg-config
    rust-analyzer
    rustc
    rustfmt
    sqlite
  ];

  linuxDiagnosticPackages = pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [
    pkgs.lsof
    pkgs.usbutils
    pkgs.util-linux
  ];
in
pkgs.mkShell {
  packages = commonPackages ++ linuxDiagnosticPackages;

  shellHook = ''
    printf '%s\n' \
      "koreader-companion dev shell" \
      "Rust tools are ready for the upcoming headless workspace." \
      "Suggested commands once code lands: cargo build | cargo test | cargo fmt | cargo clippy" \
      "Linux diagnostics: lsusb | lsblk | blkid"
  '';
}
