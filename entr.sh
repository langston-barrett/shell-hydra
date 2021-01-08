#! /usr/bin/env nix-shell
#! nix-shell -i bash -p entr

find src -name "*.rs" | entr -c -s "cargo build && bash ./fmt.sh && echo 'All clear!'"
