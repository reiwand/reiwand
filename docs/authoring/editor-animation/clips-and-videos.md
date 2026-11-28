# Clips and videos

On this page: clips of skeletons and models, their keys, events and blending, and videos.

## How to edit a clip of a skeleton or a model

A clip is a named animation of the bones of a layer with a skeleton or of a 3D model. A layer can have several clips (walk, wave...). Each clip has a duration, a repeat mode, keys per bone and events.

Where it is:

- On a layer with a skeleton: the layer's inspector, bones section, the **Edit clips on the timeline** button (that section also has the **Clip in the preview** list). If the layer has no clips, it creates one called “clip_1” and opens it.
- On a 3D model: the inspector's **Animation** section. Each clip appears as “name, 2 s from file.glb” (or “from its own keys” if it does not come from the file) with the **On the timeline** button.
- With the clip open, the time strip changes: at the top the clip's bar, below it the events row and below that one row for each channel.

Steps to create and organize clips (the clip's top bar):

1. The selector on the left changes the clip.
2. **Add a clip** (plus icon) opens a menu: **Add a clip** or **Add a clip and play it in a new animation layer**.
3. **Rename the clip** and **Remove the clip** act on the open clip.
4. **Length** is the duration in seconds (with one decimal).
5. The repeat selector offers **Repeat**, **Back and forth** and **Once**.
6. **Back to the timelines** (close icon) returns the strip to the timelines mode.

A 3D model with animations in its file also has **New clip from the file…**: choose an animation from the file and a clip with its name is created, already open.

In the terminal: the `clip` action (create, rename with `to`, `length`, `loop`, remove, `animation` to take it from the model's file, `play` for a new animation layer). The `motion` query of `scene ask` to see a layer's clips. See [actions](../actions.md).

## How to put keys on a bone, a shape or an order

Where it is: the time strip with a clip open. Each row is a channel:

- A bone has a channel for each thing it moves: on a mesh, rotation, position and scale; on a 3D model, angles and offset. The row is called “bone · channel”.
- **Shape X**: the weight of a blend shape, from −1 to 1. It has a bar to set it at the current instant.
- **Order of X**: the drawing order of a part, with a number field.

Steps:

1. Choose the bone (on the canvas or in the bone list). Its rows appear in bold.
2. Move the playhead to the desired instant with the clip's bar (**Play** / **Pause**, the instant and the bar).
3. Place the bone in the pose you want.
4. Click **Key** (key icon). It is only possible if a bone is chosen. It stores the bone's current pose.
5. For a shape or an order, change its bar or its field in the row: it puts a key at the current instant.
6. To move a key, drag it along the row. With Ctrl held when starting to drag, the key is copied instead of moved (the help says “Copy to 1.25 s” or “Move to 1.25 s”).
7. Click a key to choose it (the playhead jumps to it). A key shaped like **■** is of jump type; the others are **◆**.
8. The curve selector of the clip's bar changes the smoothness of the span, as on timelines. If you drew the curve by hand, the selector says **Custom curve**.
9. **Curve of the key** opens **Curve to the next key**, with the two handles. **Remove the key** deletes the chosen one.
10. Click on a row's bar, away from the keys, to take the playhead to that instant.

The **Curve editor** of the clip's bar opens the keys of the chosen bone (or of all), with all the tools of the previous section.

In the terminal: the `key` action (a key of a channel: `bone`, `shape` or `order`, `time`, the value and `ease`; `from` moves, `copy` copies, `remove` removes). The `keys` query of `scene ask` to list a clip's keys. See [actions](../actions.md).

## How to put events on a clip

Where it is: the **Events** row of the open clip, below its bar.

Steps:

1. Double-click the row, at the instant where you want the event. It is created with the name “event_1” (or the next free one).
2. Drag it to move it. When the pointer hovers you see its name and its instant.
3. Click it: the playhead jumps to it and a box opens with **Name of the event**, **Rename** and **Remove**.

When the clip passes the mark, the event with that name fires.

In the terminal: the `clip_event` action (`name`, `time`, `at`, `to`, `remove`). See [actions](../actions.md).

## How to blend clips (animation layers)

Where it is: the inspector of the layer with a skeleton or of the model, **Animation layers** section.

Steps:

1. First add a clip. If there is none, the text says “Add a clip first; each layer plays one clip and they mix in order”.
2. Choose a clip in **Add a layer playing…**.
3. On each animation layer: choose the clip, check **On** to enable it and **Adds** so that it adds to the previous ones instead of replacing them.
4. The bars adjust **Weight** (0 to 1), **Speed** (0 to 4), **Fade in** and **Fade out** (in seconds).
5. **Only these bones** limits the layer to the bones you choose.
6. The up arrow raises the layer in the blend order and the trash icon removes it.
7. To see a single clip in the preview, the **Clip in the preview** list of the layer with a skeleton chooses one (or **None**).
8. In a clip with motion, **Root motion: the clip moves the layer** makes the clip's root move the whole layer.

In the terminal: the `animate` action (`layer`, `root`, `preset`, `morph`). See [actions](../actions.md).

## How to trim a video and change its speed

Where it is: the inspector of a video layer (or animated image), **Video** section. Below the title you see the size (“1280 × 720 px”) and the duration (“8.00 s long” or “length unknown”).

Steps:

1. Drag the left and right handles of the bar: they mark the **Start** and the **End** that play. When you drag a handle you see “Start 1.50 s” or “End 6.00 s”. The part outside stays dark. The bar shows frames of the video to orient you.
2. **Speed** (number field, from 0 to 4 times, “1.00×”): a video plays at 0 to 4 times its speed. It is disabled if the speed is already an animated value.
3. **Loop** makes it start again when it ends.
4. **Use the size of the video** adjusts the layer's frame to the video's dimensions.
5. **Play the whole video** removes the trim.

In the terminal: the `film` action with `trim`, `rate` or `fit`. See [actions](../actions.md).

## How to see the videos' own clock in the time strip

Videos with their own clock (the video layer's playback option, which the editor offers only where it applies) have a strip in the time area.

Where it is: below the timelines, one row per video with the label “layer · own clock”.

What you see and what you can change:

1. The strip shows eight frames spread across the video, the playback span as a bar with two handles and a red mark where the clock is at the playhead.
2. Drag the handles to change the start and the end. When you release, it is saved (“Start 1.00 s”, “End 4.00 s”).
3. The “1.00×” field changes the clock's speed (“Speed of this clock”).
4. **Loop** and **Plays** turn on the repeat and the playback of that clock.

In the terminal: `reiwand-ctl scene ask DIR clocks` prints the same strips (with `--strip` it saves the frames); values are changed with `set` on the clock's pointer. See [actions](../actions.md) and [cli](../cli.md).
