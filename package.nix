{ inputs, ... }:
{
  perSystem =
    { pkgs, system, ... }:
    let
      rustPlatform = pkgs.makeRustPlatform {
        cargo = inputs.rust-overlay.packages.${system}.default;
        rustc = inputs.rust-overlay.packages.${system}.default;
      };
    in
    {
      packages.default = rustPlatform.buildRustPackage {
        pname = "devblog";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        buildInputs = with pkgs; [
          pkg-config
          openssl
        ];

        meta = {
          description = "Simple personal blog built with Rust, Axum, HTMX, and Maud";
        };
      };
    };
}
