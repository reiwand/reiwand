# Everyday commands

`reiwand-ctl` sets and controls wallpapers from the terminal. This page lists the common commands,
the fit modes, and where the configuration lives.

## Common commands

If whatever reads the output of `reiwand-ctl` stops reading early (for example `| head` or
`| grep -q`), `reiwand-ctl` ends without an error or a notice.

Set, fit and clear a wallpaper:

```sh
reiwand-ctl set ~/Pictures/wallpaper.jpg           # all screens
reiwand-ctl set ~/Videos/sea.mp4 -o DP-1 -f fit    # one screen, fit mode
reiwand-ctl fit center                             # only the fit mode, same wallpaper
reiwand-ctl fit tile -o DP-1                       # the fit mode of one screen
reiwand-ctl background '#203040'                   # color around the wallpaper
reiwand-ctl clear                                  # removes the wallpaper and gives the screen back to the desktop (on Plasma: background color)
reiwand-ctl plasma-back                            # on Plasma: back to Plasma's own wallpaper type
reiwand-ctl clear -o DP-1 && reiwand-ctl restore -o DP-1   # remove the wallpaper of a screen and recover it
reiwand-ctl filter warm --strength 0.6 -o DP-1      # color filter of a screen (none removes it)
reiwand-ctl filter --list                          # the available filters
```

Playback, sound and battery:

```sh
reiwand-ctl pause | resume | toggle                # pause the animated wallpapers
reiwand-ctl volume 40 | mute | unmute              # sound of videos and scenes
reiwand-ctl sound off -o DP-1                       # the wallpaper seen there, without sound wherever it is shown (on, follow); --volume 30 lowers it
reiwand-ctl lib apply UID -o DP-1 --sound off      # apply a video already without sound (that wallpaper remembers it; it is a local setting, not part of the package)
reiwand-ctl config pause.on_battery below:20      # pause on battery below 20 % (never = never, always = always)
reiwand-ctl config playback.lowest_below_percent 30  # lowest scene quality on battery below 30 %
```

Lists, settings and state:

```sh
reiwand-ctl playlist add Sea --folder ~/Photos/Sea --follow   # whole folder; follows the folder
reiwand-ctl playlist rotate --folder ~/Photos/Sea --every 10m   # creates the list, sets the time and starts it
reiwand-ctl config                                 # see the settings
reiwand-ctl config transition.duration_ms 1000     # change a setting
reiwand-ctl state                                  # state as JSON
reiwand-ctl gpu                                    # graphics cards and the one in use
reiwand-ctl gpu dedicated                          # draw with the dedicated one from the next start
reiwand-ctl quality q3                             # fixed scene quality (auto, q0-q3, min-q1, min-q2)
reiwand-ctl reload                                 # re-read the configuration
```

Lock screen and updates:

```sh
reiwand-ctl lock set ~/Pictures/calm.jpg           # the lock screen shows a library wallpaper of its own
reiwand-ctl lock playlist Sea -o DP-1              # a list on the lock screen of one screen
reiwand-ctl lock show                              # what each lock screen shows
reiwand-ctl lock follow                            # every lock screen shows the desktop again
reiwand-ctl lock desktop -o DP-2                   # DP-2 keeps its desktop while the others show the lock choice
reiwand-ctl update auto off                        # the AppImage stops its daily check for a new version (on, status)
reiwand-ctl update allowed --auto                  # exits with 0 when that check may run now (not in private mode, setting on)
```

## Fit modes

The fit modes are `fill`, `fit`, `stretch`, `center`, `tile` and `span`.

`span` shows an image, a video or a scene over all screens, with their physical size and the bezels
between them.

A spanned scene is drawn on each screen only in its part, with its own quality level and a single
clock for all. Its animations stay in step from one screen to another.

If the author made a scene for several screens (`display.span: "all-outputs"`), the app proposes
`span` when applying it and shows where the screens are. Each screen row lets you change it and
opens the same layout. `reiwand-ctl lib apply ID --span` does the same from the terminal.

Outputs that do not say where they are (streams, previews or screens that the desktop does not
place) show the whole image or scene. The exception: the app's screen layout places them by hand
on the board, or makes them show the same part as another output (`layout.monitors.<output>` and
`layout.follow.<output>` in the terminal).

### Memory use

- Screens that show the same image with the same edit, such as those of `span` or those with their
  own crop, share a single copy on the GPU.
- Large JPEGs are decoded directly to a reduced size.
- With a scene, each image of the package is uploaded once and used by all the screens that show
  it. The decoded copy leaves memory as soon as it is on the GPU.
- Shaders are compiled when the scene loads.
- A stable frame creates no resources and reserves no memory of its own. This is also true when
  evaluating the scene, which reuses the lists of the previous frame.

## Settings

Settings cover the transition (`fade` or `none` and its duration), frames per second and when to
pause (window on top, battery, lock).

An unknown setting ends with an error and shows the list of valid settings.

## The configuration file

The configuration lives in `~/.config/reiwand/config.toml` and can only be read by its user.

- If it is edited by hand, out-of-range values are corrected when reading it.
- If it has a syntax error, the daemon starts with the default values and shows the error in
  `reiwand-ctl state` (`config_error`). The broken file is copied to `config.toml.broken` before
  another is saved.
- When saving a change (from the app, the tray, `reiwand-ctl config` or the phone), the file keeps
  its comments, its blank lines, the order of the keys (also inside each provider) and how the
  values are written. Only what changed is rewritten.

## Test streams

For tests, `reiwand-ctl stream <output> <width> <height>` creates a stream that stays alive when the
command exits. `reiwand-ctl release <output>` destroys it.

With `--wait`, the stream lasts as long as the command does, like that of a desktop.
