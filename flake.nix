{
  description = "About sahifaga taqlid";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11"; 
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer  #(IDE uchun)
          ];

          # ixtiyoriy: muhit o'zgaruvchilari
          RUST_LOG = "info";
        };

        # ixtiyoriy: loyihani yig'ish (nix build)
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "fract_about_clone";
          version = "0.1.0";

          src = ./.;

          nativeBuildInputs = with pkgs; [ rustPlatform.cargoSetupHook ];
          buildInputs = with pkgs; [ rustc cargo ];

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/fractal_about_clone $out/bin/
          '';
        };
      }
    );

}