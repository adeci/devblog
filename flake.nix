{
  description = "Simple Rust blog site made with Axum + Rust + Maud and built with Nix!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    let
      # For importing into other flakes, export just the package
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    in
    {
      packages = forAllSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "devblog";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildInputs = with pkgs; [ pkg-config openssl ];
            meta.description = "Simple personal blog built with Rust, Axum, HTMX, and Maud";
          };
        });
    } //
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        buildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
        ];

        devInputs = with pkgs; [
          cargo-watch
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs ++ devInputs;

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

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "devblog";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildInputs = buildInputs;

          meta = with pkgs.lib; {
            description = "Simple personal blog built with Rust, Axum, HTMX, and Maud";
          };
        };

        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/devblog";
          };
          dev = {
            type = "app";
            program = "${pkgs.writeShellScript "dev-script" ''
              ${pkgs.cargo-watch}/bin/cargo-watch -x run
            ''}";
          };
        };
      });
}
