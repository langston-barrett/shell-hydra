{ pkgsOld ? import <nixpkgs> { }
, path ? ./19.09.json
}:

let
  nixpkgs = builtins.fromJSON (builtins.readFile path);

  src = pkgsOld.fetchFromGitHub {
    owner = "NixOS";
    repo  = "nixpkgs";
    inherit (nixpkgs) rev sha256;
  };

in import src
