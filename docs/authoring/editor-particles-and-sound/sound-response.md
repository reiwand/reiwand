# Sound levels and shortcuts

On this page: levels, the response, smoothing, pointer links and keyboard shortcuts.

## How to watch the sound levels

**Where it is.** The field's link menu, **Try and shape the response**, **Watch the levels of the sound…**. It can only be used with a field that follows the sound.

**Steps**

1. Choose **Watch the levels of the sound…**. The **Levels of the sound** dialog opens.
2. The bars are the bands of the sound the preview hears, about ten times a second. The bands the link reads are marked with a border and a stronger color.
3. A progress bar shows the value the link gives, and a line says “The binding gives X now (Y before shaping)”. “Before shaping” is the value without the **Limits**, **Exponent**, **Scale**, **Offset**, **Clamp to** and **Invert** settings.
4. Close the dialog when you finish: when it closes, the editor stops reading the levels.

The dialog can warn of two things:

- **The scene may not follow the sound yet. Allow the sound input of the package to see the levels.** The package must declare the sound input and the person using it must grant it. The notice brings the **Allow the sound** button: it grants the input in one click (the desktop may ask to confirm it in a notification). If the scene being shown does not use sound yet, the dialog says so: link a value to the sound, save and show the scene on a screen. See [package](../package.md).
- **Waiting for sound. Play something and leave the preview running; the levels are only read while this window is open and nothing is recorded.**

The levels are those the scene itself would hear. Nothing is recorded, saved or sent.

**In the terminal.** `reiwand-ctl scene ask DIR meter` gives the bands, the marked part, the value and its unshaped value (arguments `source`, `reading` and `of`). See [actions](../actions.md).

## How to shape the sound's response

**Where it is.** The field's link menu, **Edit the source…**. The inspector shows the link's fields.

**Steps.** Adjust the fields you want:

- **Sound**: the level or band that is followed.
- **Band count**, **Combine bands** (**Average** or **Highest**) and **Channel**: the same as the band selector.
- **Limits**: from which sound value to which value the response goes.
- **Exponent**: curve of the response.
- **Scale** and **Offset**: multiply and add to the value.
- **Clamp to**: does not let it go past those ends.
- **Invert**: flips the response.
- **Smoothing**: see the next section.

See the effect live with **Watch the levels of the sound…**: the progress bar shows the value already shaped.

**In the terminal.** The `set` action on each field of the link. See [actions](../actions.md).

## How to smooth the changes

**Where it is.** The field's link menu, **Try and shape the response**, **Smooth the changes…**. It is available on the links of a number, a pair of numbers or a color.

**Steps**

1. Choose **Smooth the changes…**. The link gets a smoothing of 0.2 s and the inspector opens it.
2. Move the **Seconds** control (from 0 to 5) or type the value (up to 60).
3. For the smoothing to differ going up and going down, turn on **Different speed going up and down**. **Going up (s)** and **Going down (s)** appear.
4. The **Response of the smoothing to a step up and down** graph shows how the value reacts to a jump. Below you read “Settles in X s going up and Y s going down”, which are three times the constant.

While the link is chosen, the **Curves** panel shows the same graph and controls on its **Smoothing** tab, with more room; a change there is the same change. With no link chosen, Curves is as before.

**In the terminal.** The `set` action on the link's `smooth` field. `reiwand-ctl scene ask DIR response` gives the graph's points (argument `smooth`, a number or `{ "up", "down" }`). See [actions](../actions.md).

## How to test a link to the pointer

**Where it is.** The field's link menu, **Try and shape the response**, **Try the pointer**. It only appears on fields that follow the pointer.

**Steps**

1. Choose **Try the pointer**. A made-up pointer travels over the preview.
2. To stop it, remove the check from the same menu entry.

To link a field to the pointer use **Follow the pointer** in the link menu: **Horizontal position**, **Vertical position**, **Over the screen** or **Speed**.

**In the terminal.** `reiwand-ctl scene ask DIR wander` gives the made-up path (argument `time`). See [actions](../actions.md).

## Keyboard shortcuts

None of these screens defines shortcuts of its own.
