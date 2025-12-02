{
  description = "lt-rs Development Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            rustfmt
            clang
            clang-tools
            cargo
            rustc
            rust-analyzer
            lld
            boost
            boost-build
          ];

          buildInputs = with pkgs; [
            libcxx
            openssl
          ];
        };
      }
    );
}
