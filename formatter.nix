{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem =
    { pkgs, ... }:
    {
      treefmt.projectRootFile = "flake.nix";

      treefmt.programs.nixfmt.enable = true;
      treefmt.programs.nixfmt.package = pkgs.nixfmt-rfc-style;
      treefmt.programs.deadnix.enable = true;

      treefmt.programs.rustfmt.enable = true;

      treefmt.programs.prettier = {
        enable = true;
        includes = [
          "*.md"
        ];
      };

      treefmt.settings.global.excludes = [
        "*.lock"
        "target/*"
        "result/*"
      ];
    };
}
