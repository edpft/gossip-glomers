{
  description = "A very basic rust flake";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";

      overlays = [ (import rust-overlay) ];

      pkgs = import nixpkgs {
        inherit system overlays;
        config = {
          allowUnfree = true;
        };
      };

      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    with pkgs;
    {
      formatter.${system} = nixpkgs-fmt;

      devShell.${system} = mkShell {
        nativeBuildInputs = with pkgs; [
          cargo-watch
          jdk17
          graphviz
          gnuplot
        ];
        buildInputs = [ rustToolchain ];
      };
    };
}
