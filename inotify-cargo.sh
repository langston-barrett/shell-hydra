#! /usr/bin/env nix-shell
#! nix-shell -i bash -p inotify-tools

inotifywait -r -m -e modify src/*.rs |
  while read path _ file; do
    clear && cargo build && bash ./fmt.sh && echo "All clear!"
  done
