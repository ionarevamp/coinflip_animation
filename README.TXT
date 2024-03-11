Build/Run with `cargo (build|run) --release [-- [DURATION] [COINSIZE]]`

Recommended: create shortcut (via symlink, alias, PATH variable, etc.) as `cfanim` or `coinflipanim` or the like for ease of use.

Once built, the extra arguments can follow the executable as normal like so:
`path_to_exe [DURATION] [COINSIZE]`

DETAILS:

The actual animation in question will be centered on the screen, and lock the terminal for the duration of the animation.

Also (currently), the program does not handle an interrupt gracefully so your cursor will stay hidden if you exit before it finishes.

This program is not intended for serious use.
