{
  description = "editor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils }: let 
  in flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
    };
   in {
    devShells.default = pkgs.mkShell {
      buildInputs = [
        pkgs.cargo
        pkgs.bacon
      ];

      shellHook = ''
        export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
      '';
    };
  });
}
