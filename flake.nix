{
  inputs = {
    holonix.url = "github:holochain/holochain";

    holonix.inputs.versions.url = "github:holochain/holochain?dir=versions/0_1";

    nixpkgs.follows = "holonix/nixpkgs";
  };

  outputs = inputs@{ holonix, ... }:
    holonix.inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      # provide a dev shell for all systems that the holonix flake supports
      systems = builtins.attrNames holonix.devShells;

      perSystem = { config, system, pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ holonix.devShells.${system}.holonix ];
            packages = with pkgs; [
              # add further packages from nixpkgs
              nodejs-18_x 
              nodePackages.pnpm
              cargo-watch
            ];
          };
        };
    };
}
