{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/x86_64-linux";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} (import ./flake);
}
