# Media player

A scene can show what is playing in a player on the computer: the title, the artist, the album,
the cover, its colors, whether it is playing or paused and how far along it is. The package
declares the `media` input (`inputs` of the manifest; the validator requires it to declare it if
it uses it) and nothing reaches it until you allow it for that package: the permissions notice of
the app and of the tray, the *Show what the media player plays* switch in its settings or
`reiwand-ctl input grant <id> media`. Without permission it receives empty texts and whatever the
author set for when nothing plays. The service listens to the players over the session bus
(MPRIS) only while a visible screen shows a wallpaper with that permission that uses it; it
follows their signals without polling, chooses the one that is playing (or the last one that
played) and stops listening as soon as nothing visible needs it. Nothing it reads is kept, logged
or sent anywhere. *Settings › Device › What you are listening to* and `reiwand-ctl media status` say whether it is being
read, for which screens and whether a player is open, without showing the track. If listening
fails (for example, the bus is lost), it is marked as not read and starts again after 10 seconds
while a visible wallpaper needs it.

Some players give the cover as an internet address instead of a file. It is only downloaded with
*Fetch covers from the internet* on (`reiwand-ctl config media.remote_covers true`, off by default
and specific to each computer), at most 4 MiB and in 5 seconds, and that website can learn that
this track is playing here; covers only live in memory. When it is turned on, the cover of what is
playing is downloaded right away and when it is turned off, those that came from the internet are
removed. Each address is downloaded only once: the last eight covers from the internet are
remembered in memory while the switch stays on, so going back to a track does not ask for it
again.

In the editor, the link menu of each field has *Follow the media player* with the data that fit:
the title, the artist, the album, the album artist, the subtitle, the genres and the type for a
text, each with what it would show now. *Text while nothing plays…* opens the text it shows
while there is no track or the permission is missing (`placeholder`, one text or one per
language, at most 512 characters), with its translations; from the terminal, `reiwand-ctl scene
act FOLDER set` and `reiwand-ctl scene act FOLDER translate` (the JSON of each one, with
`reiwand-ctl scene actions NAME`). The *Player* tab simulates a track (title, artist, album,
whether it is playing and cover) so you can design without music playing; only the preview sees
it, without asking for permission or reading the players. From the terminal, `reiwand-ctl scene
preview FOLDER --media track.json` does the same with a JSON of those fields.

The *Preview quality* button of the canvas bar draws the preview as the daemon draws it at Q0, Q1,
Q2 or Q3, on battery and with less motion, and for each effect says what is left out and how much
it costs at that level. There too it is compared with another level or without the glow of the
canvas (holding the button down shows the other side), an image or a clip is saved to a file,
with a simulated pointer if needed, and the effects and emitters that follow the pointer are
named, to try them with *Test the pointer and clicks*, which now draws a ring where the pointer is
and a flash on each click. From the terminal, `reiwand-ctl scene preview FOLDER --battery --calm`
and `reiwand-ctl scene render FOLDER --out file` do the same.

The *Test* tab (also on page packages) shows the user properties of the manifest with the same
control as the settings of a shown package and lists their presets; choosing a preset or changing
a value only changes the preview, without saving anything or touching any screen, and *Back to the
defaults* ends the test. From the terminal, `reiwand-ctl scene preview FOLDER --values
values.json [--preset ID]` does the same.

The cover is an image resource with `media: current` (that of what is playing) or `media:
previous` (the previous one), which is valid wherever an image is; its file is what is seen while
nothing plays or without permission. The image picker of the editor offers *Cover of what plays*
and *Previous cover*, made from the chosen image, and lets you choose that replacement image;
from the terminal, `reiwand-ctl scene asset FOLDER cover --cover current --placeholder IMAGE`,
and with `--field /scene/layers/0/image` it puts it in that field in a single step, using a cover
that already exists or creating another, never converting a normal image. Covers are read once
per track, reduced to 256 pixels at most and only live in memory.

Five colors come out of each cover, computed once: the primary, the secondary and the tertiary
(the most frequent), one for text that reads well over the primary and a high-contrast black or
white: `{"media": "primary"}`, `secondary`, `tertiary`, `text` and `contrast`, with `smooth` to go
from one to another without jumps. The *Player* tab shows them as color swatches.
`{"media": "playing"}` is 1 (or true) while it plays and 0 when paused, stopped or without a
track, and is useful for showing or hiding layers or for any number. `position` and `duration`
(seconds) and `progress` (from 0 to 1) are useful for progress bars and, in a text, are written
as `3:25` or `42%`. The position advances by itself from the last reading of the player, without
asking it again: a bar asks for 30 frames per second at most and a text one per second, only while
it plays.

A timeline or a sound layer can also start by itself, without scripts, with `media:play` (starts
playing), `media:pause` (pauses or stops), `media:track` (a new track arrives) or `media:cover`
(a new cover arrives). The trigger picker of the editor offers them by name, next to the named
events of the timelines and clips (`event:`) and the click, enter and leave of the pointer of
each solid or touchable-bone layer (`click:`, `enter:`, `leave:`); the *Next track* button of the
*Player* tab tests them in the preview and from the terminal `reiwand-ctl scene act FOLDER
timeline` sets them (`reiwand-ctl scene actions timeline` shows its fields).

Scripts that declare `media` read the `media` object of the `reiwand` module (state, track texts,
whether there is a cover and its colors, position, duration and progress) and receive
`mediaStatus`, `mediaPlayback`, `mediaProperties`, `mediaThumbnail` and `mediaTimeline` when
something changes; without permission they only see `enabled: false`, and never receive the cover
image. *New script…* in the resources panel (or `reiwand-ctl scene act FOLDER import_script`)
creates a module with all those events and values to link; there are other templates (`touch`,
`timer`, `audio`, `spawn`, `wave`, `daytime`, `setting` and `launches`). The *Scripts* section of
the resources panel lists the modules of the scene and lets you remove them, order them, bind a
`.js` from the folder, open them in the system editor (the preview reloads by itself when the file
is saved) or edit them in the app itself, with an API reference. A report compares the code with
its declaration (what it uses and does not declare, what it declares and does not use) and
*Declare what it uses* fixes it. The `scene.spawn` templates come out as a tree of layers with
their inspector, and *Make a template* takes the chosen layers. To try them without music,
`reiwand-ctl media simulate SCREEN track.json` shows an invented track on any screen (`--stop`
removes it).

A web wallpaper that declares `media` receives the same with `window.reiwand.onMedia(cb)`: state,
title, artist, album, the cover as a `blob:` address made on the page itself, its colors, position
and duration. If it can also reach the internet (it declares network origins), it asks for a
second, separate confirmation (`reiwand-ctl input grant ID media_network`, the second notice of
the app and the tray, which also let you remove it in the scene controls, or on the phone a
dialog of its own when turning on the second switch), because the title and the cover could leave
over the network.

The same holds for sound: a web wallpaper that declares `audio` and network origins hears nothing
until both `audio` and `audio_network` are allowed (`reiwand-ctl input grant ID audio_network`,
the second notice of the app and the tray, or on the phone a dialog of its own when turning on the
second switch), because what it hears, the sound that plays or the microphone, could leave over
the network.
