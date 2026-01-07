{
  description = "Fractal About Clone (Relm4 + GTK4)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable."1.92.0".default;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "relm4-dev-shell";

          packages = with pkgs; [
            rustToolchain
            rust-analyzer
            pkg-config
            openssl
            gtk4
            glib
            gobject-introspection
          ];

          shellHook = ''
            export PATH=${rustToolchain}/bin:$PATH
            echo "Using rustc from: $(which rustc)"
            rustc --version
          '';

          RUST_BACKTRACE = "1";
        };
      }
    );
}
