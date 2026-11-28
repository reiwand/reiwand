# Timelines, triggers and curves

On this page: adjusting timelines, events, triggers and the curve editor.

## How to adjust a timeline (duration, repeat, speed, start)

Where it is: the **Timelines** button, then the timeline's name and **Edit the timeline and its events…**. The **Timeline** dialog opens with the name.

Steps and fields:

1. **Length in:** choose **Seconds** or **Frames**.
   - In seconds, type the duration in **Seconds:**.
   - In frames, choose how many **Frames:** the timeline has and at how many frames per second (**at** “30 fps”). The duration comes from dividing one by the other.
2. **Plays:** **Once**, **Repeating** or **Back and forth**.
3. **Blend the end into the start:** can only be checked when it repeats. It makes the jump unnoticeable when it returns to the start.
4. **Speed:** bar and text field, from −4 to 4. A negative value plays backward. The bar leaves a single undo step per drag.
5. **Delay in seconds:** how long the timeline waits before starting.
6. **Start on its own:** if checked, the timeline starts as soon as the scene loads. If not, it waits for its trigger.
7. **Started by:** choose a trigger from the list, or **Nothing**. The list has the triggers the scene knows.
8. **Fire it in the preview:** simulates that trigger in the preview. It starts this timeline and all the timelines and sounds with the same trigger, as the scene would. It is the way to test a timeline that does not start on its own.

Each change is an undo step.

In the terminal: the `timeline` action with `duration`, `frames`, `fps`, `loop`, `wrap`, `speed`, `delay`, `autoplay` and `trigger`. To see what a trigger starts, the `fires` query of `scene ask`. See [actions](../actions.md).

## How to put events on a timeline

Events are named marks on the timeline. An action or a script can wait for one by its name.

Where it is: **Timeline** dialog, **Events:** field.

Steps:

1. Double-click the dialog's strip, at the instant where you want the event. It is created with the name “event_1” (or the next free one). The help text says: **Double-click the lane to add an event there; triggers and scripts start on its name.**
2. Click the mark to choose it. Its name (change it and confirm), its instant (“at 2.5 s”) and the **Remove** button appear.
3. To move it, drag it. When the pointer hovers, you see “name at 1.5 s: drag it to move it”.

Faster, from the header of the **Timeline** panel: move the playhead and click **Trigger** (lightning icon). It adds “event_1” (or the next free name) at that instant to the scene's timeline, or, with several, to the one you choose; with no timeline it is off and says to make one in **Change…** first.

The **Timeline** panel marks the events of every timeline in an **Events** lane under the ruler, each with its name at its second; click a mark to take the playhead there. Each sound layer has a lane too, with the waveform of its sounds and what starts it.

In the terminal: the `timeline` action with `add_events`, `drop_events` and `clear_events`. See [actions](../actions.md).

## How to make something start with a trigger

A layer's or a timeline's trigger field is a selector with a test button.

Where it is: the trigger fields in the inspector and the **Timeline** dialog (**Started by:**).

Steps:

1. Open the selector and choose the trigger. **Not set** leaves the field empty.
2. Click the **Fire this trigger in the preview** button (play icon). When the pointer hovers it says what starts: “Fires X in the preview, as the scene would, starting timeline A, sound B”. If nothing starts with it, it warns: “nothing in the scene starts on it yet”.

In the terminal: the `fires` query of `scene ask` to see what a trigger starts. See [actions](../actions.md).

## How to work with the curve editor

It is for seeing the curves of animated values, moving several keys at once, and copying and pasting.

Where it is: the **Curve editor** button (graph icon) in the time strip's bar, and the same button in a clip's bar. It is enabled when there are keys to show. If there are none, the dialog says “No keys to show: add keyframes to a value first”. The **Curve editor** dialog opens.

What it shows: if a key is chosen, the animated values of that layer (in a clip, the channels of the chosen bone or row); if not, all the animated values. At most twelve values are drawn at once. Each value is split into tracks: x, y and z of a position; red, green, blue and alpha of a color; or the number alone. Each track has its checkbox below the graph to show or hide it, with the name “layer · part” and its color. Each track is drawn on its own scale. The vertical red line is the playhead.

### Choosing keys

1. A click on a key chooses it.
2. Ctrl or Shift plus click adds it to the selection or removes it from it.
3. Dragging over the background draws a rectangle and chooses the keys inside. With Ctrl or Shift they are added to those already chosen.
4. A click on the background, without dragging, empties the selection.
5. To the right of the tools you see how many there are: “3 picked”.

### Moving keys

1. Drag a chosen key: it moves in time (left and right) and in value (up and down). All the chosen ones move the same amount.
2. Vertical movement only applies if the chosen keys are of the same type and unit (for example, all positions). If not, they only move in time.
3. With Alt, the vertical drag scales the values around the middle of the chosen keys of that part, instead of moving them.
4. The two text boxes in the bar do the same with an exact number: **Stretch times ×** (needs at least two keys; it stretches from the first) and **Scale values ×** (needs at least two of the same type). Type the factor and press Enter.
5. In the graph, the grip to the right of the selection box stretches the times when you drag it.

Each drag is a single undo step.

### Curving a span with the handles

1. Choose a key. Its two round handles (in and out) appear. Those that have not been touched are seen with a dashed stroke.
2. Drag a handle: it changes its length in time and its height. The time does not go past the neighboring key.
3. **Aligned handles** (the starting mode): when you drag a handle, the other turns with it and keeps its length. **Broken handles**: each handle moves on its own.
4. **Linked parts** (the starting mode): in lists and colors, the handle drawn on a track moves the curve of all the parts. **Separate parts**: each track has its own handles, in the units of that part. When you go back to **Linked parts** with a chosen key that has them separate, all take the shape of their first part.
5. **Straighten** (with a single key chosen) leaves the span without handles of its own.

### Copy, paste and remove

| What | Button | Shortcut |
| --- | --- | --- |
| Copy the chosen keys | **Copy** | Ctrl+C |
| Paste at the playhead | **Paste at the playhead** | Ctrl+V |
| Remove the chosen keys | **Remove the picked keys** | Delete |
| Frame the chosen ones | **Fit the picked keys (F)** | F |
| Frame everything | **Fit all keys (Home)** | Home |

The shortcuts only work with the curve editor open. The copied keys also go to the system clipboard as text, so you can paste them in another editor window or in another project. If the clipboard has other text, the last keys copied in the app are pasted. If you copied keys of a single value and that value is not in the graph, they are pasted onto the chosen value.

### Moving the view

- The wheel zooms the time in or out around the pointer.
- Ctrl and wheel zooms the value in or out.
- Shift and wheel (or the horizontal wheel) scrolls the time.
- Dragging with the middle button moves the view in time and in value.
- **Fit the picked keys (F)** and **Fit all keys (Home)** adjust the view to the chosen keys or to all of them.

In the terminal: the `edit_keys` action to move, scale, stretch, curve, straighten, link, remove and paste; the `curves` queries (keys per part with handles and the sampled curve) and `key_clip` (text of the key clipboard) of `scene ask`. See [actions](../actions.md).
