{
  description = "Minimal Rust Development Environment";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    andoriyu = {
      url = "github:andoriyu/flakes";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        fenix.follows = "fenix";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { self, nixpkgs, fenix, flake-utils, andoriyu, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      cwd = builtins.toString ./.;
      overlays = [ fenix.overlay ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    with pkgs; {
      devShell = clangStdenv.mkDerivation rec {
        name = "rust";
        nativeBuildInputs = [
        (with fenix.packages.${system};
           combine [
             stable.defaultToolchain
             targets.wasm32-unknown-unknown.stable.rust-std
         ])
          rust-analyzer-nightly
          bacon
          binutils
          cargo-cache
          cargo-deny
          cargo-diet
          cargo-sort
          cargo-sweep
          cargo-wipe
          cargo-outdated
          cmake
          andoriyu.packages.${system}.dart-sass-1_52_1
          gnumake
          grpcui
          grpcurl
          nodejs-16_x
          openssl.dev
          pkgconfig
          protobuf
          rusty-man
          sqlx-cli
          trunk
          wasm-bindgen-cli
          sqlite
          zlib
          just
          curl
          jq
        ];
      };
    });
}

