# RGB lighting

A wallpaper can paint the keyboard, the mouse and other RGB lights through an OpenRGB server (with
its SDK protocol; the client is our own). A scene does it with an `adjustment` layer with `rgb:
true`, which sends what it captures (the whole canvas or the region of its `size`) reduced to 64 ×
16 at most; a web page, with `window.reiwand.rgb.setImage(bytes, width, height)`, which returns
`true` only when the colors reach the lights (or the preview) and `false` without the permission,
with lighting off or on the phone. In both cases the package declares `permissions.rgb`, it is
sent 30 times per second at most and only when the frame changes, and nothing leaves until you do
two things: turn on the lighting in *Settings › Device › RGB lights* (`reiwand-ctl config rgb.enabled
true`) and allow it for that package (the permissions notice of the app or `reiwand-ctl input
grant <id> rgb`). The server is this computer (`127.0.0.1:6742`) unless you set another address
(`rgb.server`), and then the app warns that the colors travel over the network. The same section
has *Connect*, the list of devices with their lights and *Test* on each one (a 3-second rainbow);
`reiwand-ctl rgb status`, `rgb devices`, `rgb test [N]` and `rgb strip -o OUTPUT` do the same from
the terminal. While a window covers the wallpaper that sends the colors and the wallpaper is
paused for that reason (`pause.when_covered`, the default), you choose what the lights do (*While
a window covers the wallpaper and it pauses* in the same section, `reiwand-ctl rgb covered
follow|pause` or `rgb.when_covered`): *Pause and leave the lights as they are* (the default) sends
nothing, draws and captures nothing for the lights and leaves the devices with their last colors
until the wallpaper is seen again, without returning them to their own mode so as not to make
them flicker; *Keep following the wallpaper* keeps the lights changing with the scene covered: the
scene keeps running at 30 frames per second at most, drawn at 320 pixels per side at most only
for the strip and never on the screen, without those frames changing its quality level, and a page
keeps going the same without being shown, both only while the server is connected and has devices
(if not, the covered wallpaper stays paused and the service tries the server again every 10
seconds, only while that wallpaper has the `rgb` permission, sends colors and nothing else pauses
it: never because of an image, a video, a package without colors, the locked session or the
battery; the service closes the connection, whether a covered wallpaper or one in view opened it,
as soon as nothing covered asks for the lights and no visible, running screen sends colors, on
locking, pausing, switching to battery, changing to a wallpaper without colors or removing the
screen, and keeps it while a screen in view keeps sending colors even if the last one to paint was
the covered one). If the covered wallpaper is not paused, it keeps being drawn and the lights
follow it as always, whatever you choose. `reiwand-ctl rgb covered` and `reiwand-ctl rgb status`
show the choice and whether the lights are paused now; with *Keep following the wallpaper*,
`reiwand-ctl rgb status` and the RGB section of the app also say, screen by screen, why the lights
are waiting: the wallpaper is paused, the session is locked or the computer is on battery (and the
wallpaper pauses then), the package does not have the `rgb` permission or the covered screen shows
something without colors. Each device switches to its direct mode only when colors are painted to
it and recovers its own mode (and its colors, if that mode fixes each light) as soon as nothing
sends colors: on stopping or changing the package, on removing the screen, on turning the lighting
off, on changing server, on finishing a test with nothing to paint and on stopping the service.
That own mode is saved on this computer while the device is taken, so if the service closes
suddenly or the server does not respond when it is returned, it comes back as soon as the service
reaches the server again. The device list of the app and `reiwand-ctl rgb devices` show that own
mode and whether it will return when the lights stop. Each device takes the whole image until you
place it: in the same section, the lights board shows the image they receive (the 64 × 16 strip of
the wallpaper that sends them) and you choose a device or one of its zones, drag its frame over the
image and pull at its corner to size it; you also choose which way a row of lights runs (left to
right, the reverse, top to bottom or bottom to top), *Whole picture*, *Leave the device alone*
(its mode is not touched) or *Keep the zone dark*. On the terminal it is `reiwand-ctl rgb place
"NAME" "x y width height [right|left|down|up]"` (fractions of the image; `--zone N` for a zone,
`off` or `none`; if two devices have the same name, the name carries its serial number in
parentheses, or its order if they have none, as `reiwand-ctl rgb devices` shows it), and
`reiwand-ctl rgb place` lists what is placed; it is saved on this computer and is not
synchronized. To try a web page with its lights, its card in the library has *Try the page and its
lights…*: it shows it in a preview with the strip that it would send to the lights, even if the
lighting is off or the package has no permission (there `setImage` returns `true` and the colors
are only seen); when closed it is forgotten. From the terminal, `reiwand-ctl stream preview:NAME WIDTH HEIGHT`,
`reiwand-ctl lib apply ID -o preview:NAME`, `reiwand-ctl rgb strip -o preview:NAME` and
`reiwand-ctl release preview:NAME` do the same. Each zone with a matrix (the keys) takes the cell
of the strip that falls over it and the others, the column of each light. In the editor, the
inspector of an `adjustment` layer has *Send what this layer captures to RGB lighting* (it also turns on the permission
of the package, `reiwand-ctl scene act DIR rgb`), *Whole canvas* or *A strip of it*, the region
with handles in the preview and the 64 × 16 strip that the lights receive, live. The phone has no
RGB lighting and plays those packages without it.
