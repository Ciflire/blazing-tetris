# flake.nix
{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      #       ↑ Swap it for your system if needed
      #       "aarch64-linux" / "x86_64-darwin" / "aarch64-darwin"

      rustPkgs = with pkgs; [ cargo rustc rust-analyzer rustfmt ];
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default =
        pkgs.mkShell # .override { stdenv = pkgs.clangStdenv; } # for C dev
        {
          packages = with pkgs; [ cargo rustc rust-analyzer rustfmt ];
          shellHook = "";
        };

    };
}
