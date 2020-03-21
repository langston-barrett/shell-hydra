{ pkgs ? import ./nix/nixpkgs.nix { } { }
}:

with pkgs;
let this = callPackage ./default.nix { };
in mkShell {
  buildInputs = [ emacs git ncurses.dev rls rustfmt zsh ];
  inputsFrom = [ this ];
}
