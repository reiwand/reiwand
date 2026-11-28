# Inputs

Audio analysis, the pointer and script outputs reach the renderer as plain structures of
`reiwand-scene`: `AudioFrame` (64 bands per channel and their average, `bass`, `mid`, `treble`,
`level`, `beat`, `silent`; all zero while audio is off), `PointerState` (position, smoothed
position, velocity, buttons, `inside`, in fractions of the screen) and `ScriptOutputs` (values of
script sources, layer and effect overrides, copies of templates and their order, timeline,
particle and sound commands, the next timer, console lines and stopped modules), with the presses
(`Press`: a place and the buttons held after it, at most 16 between two frames) that the scene
turns into pointer events of its `solid` layers (`Touch`). A scene only renders again for an input it
uses. The pointer follows the reported position with a critically damped spring that settles in
about 0.15 s and returns to the center when the pointer leaves the screen. Audio and the pointer
need the user's grant for the scene (`dbus.md`, `Inputs1`), which the app and the tray ask for once
when such a scene is shown; a `reiwand.wpak/3` manifest declares them in `inputs`
(`package.md`), and a scene of such a package receives only the inputs its manifest declares. What
the media player plays (`media` values, cover assets, `media:` triggers and scripts that declare
it) is the `media` input, and clicks on `solid` layers the `clicks` input. Clicks are off on every
screen until the user turns them on for that screen as well as granting them to the package, they
come only from the compositor or the desktop (never from another program), and keys never reach a
scene. On phones a tap on the home screen is a click, with the package's *React to taps* switch.
