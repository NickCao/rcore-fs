{
  inputs = {
    nixpkgs.url = "github:NickCao/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = import nixpkgs { inherit system; }; in with pkgs;
        {
          devShells.default = mkShell {
            nativeBuildInputs = [ rustc cargo rustfmt rust-analyzer pkg-config ];
            buildInputs = [ fuse3 ];
          };
        }
      );
}
