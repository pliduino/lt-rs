{
  description = "Aurora Development Shell";

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

        dlopenLibraries = with pkgs; [
          libxkbcommon
          vulkan-loader
          wayland
        ];
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            rustfmt
            clang-tools
            cargo
            rustc
            rust-analyzer
          ];

          buildInputs = with pkgs; [
            libcxx
            openssl
          ];

          nativeBuildInputs = with pkgs; [
          ];

          env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibraries}";
        };
      }
    );
}
