{
  description = "ZMK Protocol Dev Shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs { inherit system overlays; };

    inherit (pkgs.lib) concatStringsSep sourceFilesBySuffices;

    rust = pkgs.buildPackages.rust-bin.stable.latest.minimal;
    cargoTemplate = ./zmk-protocol-rs/Cargo.toml;

    src = sourceFilesBySuffices ./proto/zmk [".proto"];
    protoFiles = concatStringsSep " " (pkgs.lib.filesystem.listFilesRecursive src);
    buildInputs = with pkgs; [
      rust
      protobuf
      protoc-gen-prost
      protoc-gen-tonic
      protoc-gen-prost-crate
    ];

    app = lang: out: pkgs.stdenv.mkDerivation {
      name = "zmk-protocol-${lang}";
      version = "0.1.0";

      inherit src buildInputs;

      buildPhase = ''
        set -vox
        mkdir -p gen/src
        protoc --prost_out=gen/src \
          --tonic_out=gen/src \
          --prost-crate_out=gen \
          --prost-crate_opt=gen_crate=${cargoTemplate}\,no_features=true \
          -I ${src} ${protoFiles}
      '';

      installPhase = ''
        cp -r gen/src $out
        mv $out/lib.rs $out/mod.rs
      '';
    };
  in {
    environment.variables = {
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    };

    packages.${system} = rec {
      default = rust;
      rust = app "rs" ./zmk-protocol-rs/src/proto;
    };

    devShells.${system}.default = pkgs.mkShell {
      inherit buildInputs;
    };
  };
}
