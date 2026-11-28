# See all, Several chosen, Damaged or missing file, No results and failures, First start, Options of a wallpaper, My presets, Permission question, Still on its poster and Damaged scene

On this page: see all, several chosen, damaged or missing file, no results and failures, first start, options of a wallpaper, my presets, permission question, still on its poster, damaged scene.

## See all

The same collection in a grid with an even veil over the image; the card of the wallpaper with
focus stays fixed at the bottom. First comes what is set now: the wallpaper or wallpapers in use on
each screen (with their icon), then, if a list is running, its wallpapers in the order of the list
(each with the name of the list) and then the rest in the usual order, without lines or frames
between the groups. A filter or a search keeps that rule inside its results. The grid arrives with
focus on the wallpaper in use. The shelf keeps its order. In the terminal, `reiwand-ctl lib search
'first:<uid>,<uid>'` gives the same order. Esc or "Show shelf" goes back to the shelf.

## Several chosen

Ctrl+click, the circle that appears when passing over a cover or Space choose wallpapers. "Choose
wallpapers in the library" (in "Lists") opens "See all" already choosing, with a circle on each
cover; from "Wallpapers" of a list, "Add wallpapers…" does the same and adds the button "Add to
"list"", which goes back to the list with the chosen ones added. In place of the card appears the
selection line: "N chosen" ("1 chosen" with one), "Rotate these every…", "Playlist", "Tags", "More"
(favorite, rating, allow flashes, send the files to the other computers, remove) and "Stop
choosing" (Esc). Each action opens a small panel next to its button: "Rotate N wallpapers" with 10
min, 30 min, 1 h, Every day, "Other…" (seconds, minutes or hours) and "Use a folder instead…";
"Playlist" with the hand-made lists and "A new playlist…"; "Tags" with those that are added and
those that are removed; and the questions of rating, allowing flashes and removing (with "Also
delete the imported copies"). With several screens the rotation starts on the chosen screen.

## Damaged or missing file

The card says in Coral that the file could not be opened, with the word of its class (image, video,
scene or wallpaper), and offers "Show in the folder" (Enter), "Options ›", which opens the panel of
the damaged scene, and "Remove from the library…", which asks before removing; the details show
the error.

## No results and failures

A search with no results says "Nothing matches "…"" and offers "Clear the filters".

An imported wallpaper whose file is not on this computer shows a notice that says which it is,
where it comes from (an imported list, profile or repository) and what to do; if it comes from a
repository, it offers "Add the repository". Closing it hides it for that wallpaper.

## First start

With the library empty the page does not show the filter bar: it says "You have no wallpapers yet"
and offers three paths, "Explore repositories" (main), "Add a source" and "Add a folder", with the
included examples (those of the `bundled` repository) in a row below; pressing one installs it and
sets it as wallpaper. Dropping files or a folder on the page shows "Drop to add them to your
library".

The first wallpaper is set by itself: when the library is empty, no screen shows a wallpaper and the
examples repository is already there, the app installs the first one that asks for nothing (no
inputs or permissions) and does not flash, applies it to all screens with `Apply` and says so in a
notice with "Undo". Undo calls `ClearWallpaper` and leaves the desktop as it was. It happens only
once: the file `reiwand/first-run` in the configuration folder remembers it, even if the library is
emptied later. If the examples repository is not there (installation without examples) only the
three paths are seen.

If the service does not respond, the page shows only the failure, "The wallpaper service does not
answer", with "Restart the service" and the details; it retries every three seconds and comes back
by itself when the service answers. The button asks the session's systemd to restart
`reiwandd.service` (like the tray applet); without that unit it asks the bus to activate
`io.github.reiwand.Daemon`. If neither works, the text says so and the reason stays in "Details".
In the app it is the `Service.Restart` call of its table.

## Options of a wallpaper

It uses the options skeleton: the title is the name of the wallpaper, with "Favorite" and "Set as
wallpaper" at the top, and the path "Library › name" (Esc goes back). The first level shows the
preview, why the scene is still if it is, "Author options" (those of the scene on each screen where
it is set), "My presets", what the package uses, "From its author" (variants and base wallpaper),
"Yours" (tags, lists) and "More". A scene set without options of its own says "This scene has no
options of its own" and leaves "My presets" and "More".

On an image, "Yours" starts with "Depth · Move with the pointer": while the depth model is not
installed, the row says how much is downloaded. Turning it on stores in the image's edit the
parallax that follows the pointer (the same as `reiwand-ctl edit parallax --source pointer`); if the
model is missing it asks for permission with the same box as the editor (cancelling it turns it off
again) and, with the model ready, it computes the depth map on this computer. Turning it off removes
the parallax and keeps its values. Fine settings remain in Edit › Parallax. While such an image is
on a screen, Settings › Privacy lists it among what scenes can use, with its name and "Pointer",
and "Stop using Pointer" turns that parallax off (the same as `reiwand-ctl inputs revoke PATH
pointer`; `reiwand-ctl inputs status` lists it the same). Turning the switch on again asks
nothing.

The "More" level (path "Library › name › More") starts with "Permissions of this scene": one row
for each thing the scene asks for (pointer, sound, clicks, what plays and their variants with
network), with "Allowed" or "Not allowed"; changing it answers with `Inputs1.Decide`. Then come
"This scene" (sound, "Quality of this scene · Like your screen", "Edit a copy…"), the name, the
rating, the file data, "Keep moving" with reduced motion, the privacy of a web page and its lights,
"Edit…" and "Remove from the library…".

"Quality of this scene" does not set a quality of its own: quality is set per screen. The row
appears while the scene is on a screen and says, with the words of Settings, the quality of that
screen (`playback.quality.<screen>` or, if it has none, `playback.quality`) and where it is changed;
Enter opens Settings.

## My presets

The "My presets" level (path "Library › name › My presets") lists the presets of the wallpaper and
the global ones (`Profiles1.ListPresets`), with their placement and their fill. Enter uses the
preset on that wallpaper (`Library1.Apply` with the preset); "Delete…" asks for confirmation in a
panel next to the row. "New preset…" opens a panel with "Name", "Placement" and "Fill color"; Tab
goes to the next field, Esc cancels and "Save" creates a global preset (`Profiles1.SavePreset` with
an empty item).

## Permission question

When a screen shows a scene that asks for things the person has not yet answered (the `undecided`
field of `GetInputs().shown`), the app asks a single question, ""name" asks for permission to use",
with one line per thing (pointer, the computer's sound, clicks and taps, what plays, and what plays
or the sound with network), its explanation and a switch on each, on from the start: on is allow
and off is leave it out, without words that get confused with the buttons at the bottom. "Allow" at
the bottom applies what was chosen and "Without this" leaves everything out, with `Inputs1.Decide`;
the scene already works meanwhile without those permissions, and if something was left out a notice
says where to change it. Network and lights are not asked here. A preview (the running cover, the
preview window or the editor, `preview:` outputs) does not ask: the scene is seen without those
permissions until it is set as the wallpaper on a screen. It is asked once per scene in each app
session; what was answered is remembered in the service. The piece is `PermissionsQuestion` and other
screens use it with `ask(scene, key, name, things)`.

## Still on its poster

The panel says that the scene set is still in only two cases: it is too heavy for the computer with
automatic quality (`quality_auto` = `still`). In the latter case it offers "Play anyway", which sets
`playback.quality.<screen>` to `min-q1`, and in addition the app shows once a notice with the same
button. A wallpaper with flashes while they are off in Settings (`flash_disabled`) also says so; the
general setting is only changed in Settings. A pause because a window covers the screen, because of
battery, locked screen or by hand is not shown in the app: its reason is given only by the terminal
(`reiwand-ctl state`).

## Damaged scene

If the file is missing or damaged, the panel shows "Could not open this scene" with the reason,
"Install it again" when it came from a repository, "Show in the folder" and "Remove from the
library…" with confirmation. Reinstalling the same version checks the file against the size and
hash noted when it was installed: if it is missing or does not match, it downloads it again from
the repository with the same signature and listing checks as the first installation and keeps its
record and its settings; if it is intact, it downloads nothing.
