Run with `coinflip_animation DURATION COINSIZE`.
Running without arguments will default to 10 seconds and a diameter scaled to your terminal.
Running with one argument will specify the duration, and scale the diameter.
When specified, the diameter is approximately `(COINSIZE * 2)`.

OTHER DETAILS:

The actual animation in question will be centered on the screen, and lock the terminal for the duration of the animation.

Each terminal will have different spacing and character width:height ratios, so it may not appear exactly circular. If you need/want, head over to the docs, copy & change the source code to get a better visual. Make sure to add the `ctrlc` dependency with the flag 'features="termination"'.

Sending Ctrl-C will stop the animation and show the cursor again.

DISCLAIMER: This program is not intended for serious use.
