{
  description = "warp-jwt-auth-server with dream2nix packages";

  inputs = {
    dream2nix.url = "github:nix-community/dream2nix";
    nixpkgs.follows = "dream2nix/nixpkgs";
  };

  outputs = inputs @ {
    self,
    dream2nix,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
  in {
    # All packages defined in ./packages/<name> are automatically added to the flake outputs
    # e.g., 'packages/hello/default.nix' becomes '.#packages.hello'
    packages.${system}.default = dream2nix.lib.evalModules {
      packageSets.nixpkgs = inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system};
      modules = [
        ./default.nix
        {
          paths.projectRoot = ./.;
          # can be changed to ".git" or "flake.nix" to get rid of .project-root
          paths.projectRootFile = "flake.nix";
          paths.package = ./.;
        }
      ];
    };

    devShells.${system}.default = inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system}.mkShell {

      packages = with inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system}; [
        rustup
        cargo-cross
        protobuf3_20
        diesel-cli
      ];
    };
  };
}
