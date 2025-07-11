{
  mkShell,
  pkgs,
  ...
}:
mkShell {
  env = {
    RUST_BACKTRACE = 1;
    RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
  };

  nativeBuildInputs = with pkgs; [
    # keep-sorted start
    alejandra
    commitlint-rs
    deadnix
    editorconfig-checker
    keep-sorted
    lefthook
    mdformat
    nil
    statix
    taplo
    treefmt
    worker-build
    wrangler
    yamlfmt
    # keep-sorted end

    (pkgs.rust-bin.stable.${pkgs.rustc.version}.minimal.override {
      extensions = ["clippy" "rustfmt"];
      targets = [
        "wasm32-unknown-unknown"
      ];
    })
  ];

  shellHook = ''
    lefthook install
  '';
}
