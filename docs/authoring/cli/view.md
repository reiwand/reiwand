# Viewing the scene

### `scene preview`: a preview (with the daemon)

```
reiwand-ctl scene preview my-scene -o DP-1
```

It shows the scene on the desktop for 60 seconds without applying it or saving anything. It says
on which outputs it shows it and how to end it. Without `-o`, on all the outputs.

| Option | What it does |
|--------|--------------|
| `--console` | Prints what the scripts write with `console` and the scene's error, with file and line, as they come out, until you interrupt it. It works on any output, including those of your own screens, but the scene only runs (and only writes) while something is looking at that output. See `scripts.md`. |
| `--stop` | Ends the preview and says on which outputs it ended it. If the app had *Show on the desktop* turned on, it turns that off too. |
| `--restart` | Starts the scene, the scripts and the particles over again. |
| `--time S` | Holds that second fixed (up to 86400). With a negative value, the scene plays. |
| `--quality q0`…`q3` | The highest quality level of the preview. |
| `--battery`, `--calm` | Shows it as the daemon shows it on battery, or with less motion. |
| `--without-bloom` | Shows it without the canvas glow, to compare it with the glow. |
| `--values settings.json` | Tries property values, such as `{"speed": 2}`, without saving them. |
| `--preset ID` | Starts from a preset of the package before applying `--values`. |
| `--media track.json` | Makes up a player track: `title`, `artist`, `album`, `playing`, `position`, `duration`… |
| `--layout dual\|triple\|vertical\|mine --out file.png` | Draws the scene spread across several screens, without a desktop, into a file. |

### `scene apply`: build, import and show (with the daemon)

```
reiwand-ctl scene apply my-scene -o DP-1
```

```
applied: My clock 1.0.2 on DP-1, quality Q3
```

It builds the package, adds it to your library and shows it on the output (or on all of them). It
reports what it is doing on the error output (`Building the package…`, `Adding the package to the
library…`, `Showing it on the screen…`) and, if the daemon is slow because it has other jobs, it
notifies every 5 seconds. At the end it prints the title, the version, the screen and the quality
level it shows it at; the text comes out in your language. It ends with an error if the screen says
it could not show it. If you change the scene without bumping the version, the screen shows the new
version anyway. `--span` stretches it across all the screens.

To remove the wallpaper you just applied:

```
reiwand-ctl clear -o DP-1
reiwand-ctl restore -o DP-1
```

`clear` removes the wallpaper from the output (from all of them without `-o`) and leaves only the
background color; it prints nothing. `restore` brings back the one that was there, and says `There
is no earlier wallpaper to bring back.` if there is none (for example, if after removing it you
set another one). Applying another wallpaper forgets the previous one; removing twice keeps the
first. In the app they are in *Settings*, in each screen's row, as *Remove wallpaper* and *Bring
back the wallpaper*.

### `input`: simulated inputs (with the daemon)

```
reiwand-ctl input grant org.example.my-scene pointer
reiwand-ctl input touch DP-1 0.8,0.2
```

`input decide SCENE -a INPUTS -x INPUTS` answers everything the scene asks for at once (those in
`-a` are granted, those in `-x` are left out and the scene runs without them); `input grant SCENE
INPUT` grants an input (`audio`, `pointer`, `clicks`, `media`…) to the scene being shown (`SCENE` is
the `id` of its manifest, which `scene new my-scene` leaves as `org.example.my-scene`; it prints
nothing, and `input status` confirms the permission); `input touch OUTPUT X,Y` puts a made-up
pointer at that fraction of the screen (`--stop` removes it) and `input status` says what each
screen uses and what it has been granted. They are for testing scripts and properties without
touching the mouse.
