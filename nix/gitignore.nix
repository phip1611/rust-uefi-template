# Returns the gitignoreSource function which takes a path and filters it by
# applying gitignore rules. The result is a filtered file tree in the nix store.

{ sources }:

let
  gitignoreNix = import sources.gitignoreNix { };
in
gitignoreNix.gitignoreSource
