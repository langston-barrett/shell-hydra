# TODO: https://nixos.org/nixpkgs/manual/#controlling-rust-version-inside-nix-shell
{ moz_overlay ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)
, pkgs ? import ./nix/nixpkgs.nix { } { overlays = [ moz_overlay ]; }
}:

let pname = "shell-hydra";
    version = "0.1.0";
in pkgs.rustPlatform.buildRustPackage {
  inherit version;
  name = "${pname}-${version}";
  src = pkgs.nix-gitignore.gitignoreSource [./.gitignore] ./.;
  cargoSha256 = "1hpp55d8irqwj2902x3218x35wqg4mps3i17j51gsvfby9lsd5mb";
  postInstall = ''
    cp -r $src/conf $out/conf/
  '';
  meta = with pkgs.stdenv.lib; {
    description = "Quickly and easily create keyboard-driven interfaces for command line tools";
    homepage = "https://github.com/siddharthist/${pname}";
    license = licenses.mpl20;
    maintainers = [ maintainers.siddharthist ];
    platforms = platforms.unix;
  };
}
