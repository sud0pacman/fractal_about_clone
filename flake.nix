{
  description = "Relm4 loyihasi uchun ishlab chiqish muhiti";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain-ni tanlaymiz
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };

        # GTK4 va Relm4 uchun kerakli paketlar
        nativeBuildInputs = with pkgs; [
          pkg-config
          wrapGAppsHook4
          rustToolchain
        ];

        buildInputs = with pkgs; [
          gtk4
          libadwaita
          glib
          pango
          gdk-pixbuf
          atk
          gettext
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # GTK bilan ishlashda muhim muhit o'zgaruvchilari
          shellHook = ''
            export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH
            echo "🦀 Relm4 (GTK4) muhiti tayyor!"
          '';
        };
      }
    );
}
