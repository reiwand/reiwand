# Scene properties

Scene packages (`docs/specs/scene.md`) are imported and applied like any other library item. This
page covers how they are drawn, how to change their properties from the terminal and the app, and
what kinds of properties authors can describe.

## How scenes are drawn

- The daemon draws them on the GPU only when something visible changes.
- It uses fewer frames and less quality on battery or with software rendering.
- Scenes with `"clock": { "max_fps": "display" }`, like the examples, follow the screen's refresh
  rate.

## Properties from the terminal

`reiwand-ctl prop` changes the properties of the scene a screen shows; `-o` names the screen.

```sh
reiwand-ctl prop ls -o DP-1                     # properties of the scene that DP-1 shows
reiwand-ctl prop set tint '#ff8800' -o DP-1     # applied at once and saved in config.toml
reiwand-ctl prop set volume 60 -o DP-1          # own volume of a scene with sound
reiwand-ctl prop press again -o DP-1            # presses a button of the scene
reiwand-ctl prop share -o DP-1                  # the same values on all screens (--own: only this one)
reiwand-ctl prop preset load calm -o DP-1 && reiwand-ctl prop reset -o DP-1
reiwand-ctl prop reset                          # without -o: the values of all screens that show the scene
reiwand-ctl prop preset save Night -o DP-1      # saves the current values as a preset of your own
reiwand-ctl prop preset export Night -f night.json -o DP-1
reiwand-ctl prop preset import night.json --save Night -o HDMI-A-1
```

`reiwand-ctl prop set` accepts paths relative to the current folder.

## Measuring the cost of a scene

`reiwand-ctl scene bench` measures a scene without the daemon. `scripts/phone-bench.sh` runs the
same bench on the phone.

```sh
reiwand-ctl scene bench scene.wpak --size 1920x1080 --frames 300   # cost per frame, without the daemon, on the card of render.gpu
reiwand-ctl scene bench scene.wpak --screens 2   # load, first frame and memory with two screens
reiwand-ctl scene bench scene.wpak --size 1440x3040 --quality q3 --passes --shot q3.png   # each pass and the image of the last frame
reiwand-ctl scene bench scene.wpak --quality auto --paced 30   # 30 s in real time with the phone's automatic level
REIWAND_ADB_SERIAL=<serial> scripts/phone-bench.sh build && REIWAND_ADB_SERIAL=<serial> scripts/phone-bench.sh copy scene.wpak
REIWAND_ADB_SERIAL=<serial> scripts/phone-bench.sh run scene.wpak --level q3 --passes   # the same bench on the phone, in JSON
REIWAND_ADB_SERIAL=<serial> scripts/phone-bench.sh run scene.wpak --level auto --paced 30 --pace smooth   # the automatic level without lowering the pace
REIWAND_ADB_SERIAL=<serial> scripts/phone-bench.sh costs   # what the 3D stage costs on the phone: unit, models and shadow maps
```

## Properties in the app

On the *Settings* page of the app, the *Scene settings* (or *Video settings*) button of each
screen lets you:

- change its properties at once,
- apply its presets, and save, rename and delete your own,
- copy and paste the settings, or export and import them as a small file,
- decide whether the screen shares its values with the others,
- grant or withdraw the sound, pointer and click permission,
- turn that screen's clicks on or off.

Your own presets belong to the package: they survive updating it and travel in the exported file.

While a screen shares its values with the others, what is changed on it changes on all.

## Kinds of properties

Authors describe their properties in the manifest (`docs/specs/scene.md`).

| Kind | Details |
|------|---------|
| Colors | With transparency |
| Sliders | With a step, decimals, units, logarithmic scale and a box to type the number |
| Lists | With icons and translated labels |
| Texts and secret texts | Secret texts are stored in the system keyring and never reach `config.toml` or the logs |
| Layout | Separators, labels and collapsible groups |
| Buttons | Fire scene events |
| Files and folders | Chosen by the user with the file picker, videos too |

Chosen files and folders replace an image, a video, a mask or the texture of some particles of the
package without restarting the scene.

- Scripts never see the paths of the chosen files.
- A folder is only read while the scene is visible.
- A folder can include up to two levels of subfolders and limit how many files it uses.

## Conditions

Each property can be shown or enabled according to the others with conditions such as
`mood == 2 || speed > 1.5`. They are of a small language that is checked when the package is
validated and that runs no code.

## Shortcuts and quality

The format also describes shortcuts that the user chooses to open with a click, and values that
change with the quality level.

## Video packages

A video package offers:

- speed, without changing the pitch or drawing more frames,
- the span that repeats,
- its own volume,
- zoom and position.

The default values are chosen by the author, and they are applied without restarting the video.

Next: [Example scenes and sound](scene-sound-and-examples.md).
