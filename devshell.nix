{ inputs, ... }:
{
  perSystem =
    { system, pkgs, ... }:
    let
      rustToolchain = inputs.rust-overlay.packages.${system}.default;
    in
    {
      devShells.default = pkgs.mkShell {
        packages = [
          rustToolchain
          pkgs.rust-analyzer
          pkgs.cargo-watch
          pkgs.pkg-config
          pkgs.openssl
        ];

        shellHook = ''
          echo "Development Environment"
          echo "Commands:"
          echo "  dev     - cargo watch -x run (auto-reload dev server)"
          echo "  build   - cargo build"
          echo "  test    - cargo test"
          echo "  lint    - cargo clippy"
          echo "  fmt     - cargo fmt"

          alias dev="cargo watch -x run"
          alias build="cargo build"
          alias test="cargo test"
          alias lint="cargo clippy -- -D warnings"
          alias fmt="cargo fmt"
        '';
      };
    };
}
