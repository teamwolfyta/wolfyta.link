{inputs, ...}: {
  systems = import inputs.systems;

  perSystem = {
    config,
    pkgs,
    system,
    ...
  }: {
    _module.args.pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [(import inputs.rust-overlay)];
    };

    devShells = {
      default = config.devShells.development;
      development = pkgs.callPackage ./shell.nix {};
    };
  };
}
