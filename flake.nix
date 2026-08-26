{
  description = "Rust flake for building stuff";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rust-analyzer
        pkgs.clippy
        pkgs.rustfmt
        pkgs.cargo-flamegraph
      ];


      shellHook = ''
        echo "================================================================================"
        echo "rustc version: $(rustc --version)"
        echo "rust-analyzer version: $(rust-analyzer --version)"
        echo "================================================================================"
      '';
    };
  };
}
