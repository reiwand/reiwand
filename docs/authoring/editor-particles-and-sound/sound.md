# Sound layers and links

On this page: adding a sound, its waveform, fades, triggers, volume and links to bands.

## How to add a sound

**Where it is.** The **Add a layer** button.

**Steps**

1. Click **Add a layer**.
2. To import a new file, open **Layer from a file** and choose **Sound…**. They accept `.ogg`, `.opus`, `.flac`, `.wav` and `.mp3`.
3. To use a sound the scene already has, choose **Sound** in the layer list. That option is disabled until the scene has a sound file; if there is more than one, the editor asks **Which asset**.
4. The sound layer draws nothing. A scene admits up to 8 sound layers, which are mixed.

**In the terminal.** The `add_layer` action with `kind` equal to `sound` and `file` or `source`. See [actions](../actions.md).

## How to listen to a sound and see its waveform

**Where it is.** Inspector, when choosing the canvas or a sound layer: **Sounds of the scene** block (with the canvas) or **Sound** (with a layer).

**Steps**

1. Each layer shows its name (with “muted” if it is), how it starts (**Starts with the scene** or **Starts with** the trigger) and a **Listen** button.
2. Click **Listen** for the preview to play only that sound once; the other sound layers go quiet and the trigger is ignored. Click **Stop listening**, listen to another or leave the panel to return to the scene. The app's volume and mute apply as in a real wallpaper.
3. Below each sound appears its name with its duration and its **Waveform of the sound**.
4. **Sounds no layer plays** lists the sound files without a layer, also with **Listen** and waveform.
5. With the canvas selected, the **Edit the sound layer** button (arrow) opens that layer.

**In the terminal.** `reiwand-ctl scene ask DIR sounds` lists the sound layers and files; `reiwand-ctl scene ask DIR wave` gives a file's waveform (arguments `asset` and `buckets`). See [actions](../actions.md).

## How to set fade in and fade out

**Where it is.** Below each sound, over the **Waveform of the sound**.

**Steps**

1. You will see two round dots over the waveform: **Fade in** (on the left) and **Fade out** (on the right).
2. Drag each one to where you want the sound to finish rising or start falling. A line draws the rise and the fall.
3. When you release, the value is saved in seconds (from 0 to 60).
4. If the layer has several sounds, the fade in goes on the first and the fade out on the last.
5. You can also type the two values in the layer's **Fade in** and **Fade out** fields.

The format has no trim of a sound's start and end: the fades shape the audible part.

**In the terminal.** The `set` action on the layer's `fade_in` and `fade_out`. See [actions](../actions.md).

## How to choose when it plays (the trigger)

**Where it is.** The sound layer's **Play on** field.

**Steps**

1. Open the list. With no value it reads **Not set**, and the sound starts with the scene.
2. Choose a trigger: the start, the beat, a button or setting of the package, an event, a click or the pointer entering and leaving over a layer, or the media player.
3. Click the play button next to the list (**Fire this trigger in the preview**) to try it. The help says which timelines and which sounds it starts; if nothing in the scene starts with it, it says so.

**In the terminal.** The `set` action on `play_on`. `reiwand-ctl scene ask DIR fires` says what starts with a trigger. See [actions](../actions.md).

## How to adjust the volume and other fields of a sound layer

**Where it is.** The sound layer's inspector.

**Steps.** These are the layer's fields:

- **Sound** or **Sounds**: the file, or a list of 1 to 16.
- **Mode** (with several sounds): **In order** or **At random**.
- **Pause**: seconds of silence between sounds (range).
- **Stream**: reads the file while it plays, for long sounds.
- **Play on**: the trigger.
- **Loop**: repeats without a gap (on by default). If you turn it off, it plays once each time it turns on.
- **Play**: turns the sound on and off.
- **Volume** (from 0 to 1) and **Balance** (from −1 left to 1 right).
- **Fade in** and **Fade out**.

**Volume**, **Balance** and **Play** admit timeline keys and user settings (each field's link button). They cannot follow the sound analysis, the pointer or a script: a scene cannot hear its own sound.

Every scene with sound also has a volume user setting, a slider from 0 to 100 that is added by itself. The user can also mute wallpapers with sound.

**In the terminal.** The `set` action on each field. See [actions](../actions.md).

## How to link a value to the sound

A field of a number, a pair of numbers or a color can follow the sound. The editor measures it in 16, 32 or 64 bands, from bass to treble.

**Where it is.** Next to each field there is a link button: ◇ if it has a fixed value, ⛓ if it follows something, ◈ or ◆ if it has keys. Clicking it opens a menu.

**Steps**

1. Click the field's link button.
2. Open **Follow the sound** and choose one of the five named levels: **Bass**, **Mids**, **Treble**, **Level** (how much sounds in total) or **Beat** (it is 1 on each beat).
3. The field now follows that level. The link button shows “Follows Sound · LEVEL”.

To go back to a fixed value, open the menu and choose **Fixed value**.

**In the terminal.** The `set` action on the field, with the link as the value. See [actions](../actions.md).

## How to follow a band or a range of bands

**Where it is.** The field's link menu, **Follow the sound**, **A band or a range of bands…**. It opens the **Follow the sound** dialog.

**Steps**

1. The buttons at the top (**Bass**, **Mids**, **Treble**, **Level**, **Beat**) choose a named level.
2. In the band strip, click a band or drag over several. The ones you choose are marked; below you read their frequency (“Band 3: 200 Hz to 250 Hz” or “Bands 3 to 6: …”). The strip's ends show the lowest and the highest frequency.
3. In **Bands**, choose **16 bands**, **32 bands** or **64 bands**. With more bands, each one is narrower.
4. In **Several bands count as**, choose **Their average** or **The loudest**.
5. In **Channel**, choose **Both**, **Left** or **Right**.
6. A live bar over the strip shows the band levels while something sounds (it needs the sound permission, see below).
7. Click **OK** to link, or **Cancel**.

**In the terminal.** The `set` action on the field. See [actions](../actions.md).
