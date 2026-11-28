# Inputs

`inputs` declares what the package may ask the user for: the sound playing on the computer
(`audio`), the pointer (`pointer`), clicks (`clicks`), what the media player plays (`media`) and the
network (`network`, which only web pages use; other kinds that declare it get a warning). Declaring
is no grant: each input is still allowed by the user per package, when it is used, and clicks also
per screen (`dbus.md`, `SetClicks`); keys never reach a package. For scenes the
validator computes the inputs that the scene can use with any value of its properties and presets
(audio and pointer bindings, spectra, beat triggers, parallax, particles that follow the pointer
or burst on clicks, effect parameters such as the clicks of `water_ripple`) and refuses a manifest
that does not declare them, naming the missing one and the first place of `scene.json` that uses
it (`the scene uses audio at /layers/2/effects/0 of scene.json; add "audio"`).
Newer scenes are computed the same way: inverse kinematics and particle control points that follow the pointer, `enter:` and `leave:` triggers ask for `pointer`; `click:`
triggers for `clicks`; `media` values, cover assets and `media:` triggers for `media`. Scene scripts
declare in `scene.json` the parts of the script interface they use and the events they handle
(`scene.md`, "Scripts"), so their inputs are computed from that declaration rather than guessed from
their text. A second-version package receives only what it declares: the sound for `audio`, the
pointer for `pointer` or `clicks`, clicks for `clicks` and the media player for `media`, whatever its
scenes, scripts or page would read. Web pages always declare their inputs, and their facets come from that list.
On the phone `media` comes from the media sessions of the system, which Android shows to an app
only through notification access: the app asks for it with an explanation when the user turns on
*Show what plays* for a package, never reads a notification, and follows the active session by its
callbacks only while a shown wallpaper with the grant uses it; without that access the package
gets `enabled: false`, its placeholders and empty texts, as without the grant.

### RGB lighting

`permissions.rgb: true` says that the package may send colors to keyboards, mice and other RGB
lighting: a web page through `window.reiwand.rgb.setImage`, a scene through an `adjustment` layer with
`rgb: true` (`scene.md`), which the validator requires to be declared. Like the inputs it is no
grant: the user allows it per package (the `rgb` input of `Inputs1`, only from the app, the tray or
`reiwand-ctl`), and the colors go out only while RGB lighting is turned on (`rgb.enabled`, off by
default) to an OpenRGB server, on this computer unless the user sets another address (`dbus.md`,
`Rgb1`). Both name the `rgb` feature, which the desktop runs and the phone, which has no such
lighting, leaves out.
