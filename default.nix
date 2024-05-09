{
  lib,
  config,
  dream2nix,
  pkgs,
  ...
}: {
  imports = [
    dream2nix.modules.dream2nix.rust-cargo-lock
    dream2nix.modules.dream2nix.buildRustPackage
  ];

  deps = {nixpkgs, ...}: {
    inherit (nixpkgs) 
    libmysqlclient_3_2;
  };

  name = lib.mkForce "rust-warp-jwt-authenticator";
  version = lib.mkForce "0.3.1";

  # options defined on top-level will be applied to the main derivation (the derivation that is exposed)
  mkDerivation = {
    # define the source root that contains the package we want to build.

    packages = with inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system}; [
      rustup
      cargo-cross
      clang
      # Replace llvmPackages with llvmPackages_X, where X is the latest LLVM version (at the time of writing, 16)
      llvmPackages_16.bintools
      protobuf
    ];

    src = ./.;
  };
}
