{
  description = "Wrapper for plain or TLS streams";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";

    # The rustup equivalent for Nix.
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Allows non-flakes users to still be able to `nix-shell` based on
    # `shell.nix` instead of this `flake.nix`.
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, fenix, ... }:
    let
      inherit (nixpkgs) lib;

      eachSupportedSystem = lib.genAttrs supportedSystems;
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      mkDevShells = system:
        let
          pkgs = import nixpkgs { inherit system; };

          # get the rust toolchain from the rustup
          # `rust-toolchain.toml` configuration file
          rust-toolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            # cargo 1.82.0 (8f40fc59f 2024-08-21)
            sha256 = "yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
          };

        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ rust-toolchain openssl.dev ];
          };
        };

    in
    {
      devShells = eachSupportedSystem mkDevShells;
    };
}
