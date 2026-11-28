# Keys and the time strip

On this page: time, keys, the playhead, curves between keys and timelines.

## How a scene's time works

- Every scene has a **scene clock**. An animated value follows it unless you say otherwise.
- You can add **timelines** of your own. Each one has its duration, its repeat mode, its speed and its way of starting. Each animated value chooses which clock it follows: the scene's or one of your timelines.
- A **keyframe** fixes the value at an instant. Between two keys the value moves according to the **curve** of the span.
- An **event** is a named mark on a timeline (or on a clip). Triggers and scripts can start things with that name.
- A video with its **own clock** has its own strip with start, end and speed (see [How to trim a video](clips-and-videos.md#how-to-trim-a-video-and-change-its-speed)).

## How to put a key on a value

Where it is: in the inspector, next to each value that can be animated there is a small button. It shows **◇** if the value is fixed, **◈** if it has keys but none at the current instant and **◆** if there is a key right at this instant. The **⛓** symbol means the value follows something else (a property, the sound, a script...) and cannot be given keys while it follows it.

Steps:

1. Choose the layer and, with the time strip's playhead, the instant where you want the key (see [How to move the playhead](#how-to-move-the-playhead-and-play)).
2. Change the value to what you want at that instant.
3. Click the ◇ button next to the value and choose **Keyframe here**. The key keeps the value that is seen at that moment.
4. Repeat at other instants. When the value already has keys, the same menu offers **Remove the keyframe here** (active only if there is a key at the current instant).
5. To go back to an unanimated value, open the menu and choose **Fixed value**. The value keeps what is seen at that instant.

Once a value has keys, the header of the **Timeline** panel also keys it: click one of its keys in the timeline to choose its track, move the playhead and click **Key** (◇). It adds a key there with the value the inspector shows at that instant. **Key** is off until a key is chosen.

The button's menu also offers other sources for the value (user property, sound, pointer, player, script...). That is explained in [properties](../editor-properties.md) and in [particles and sound](../editor-particles-and-sound.md).

When the scene has no keys yet, the time strip says: **No keyframes yet: the ◇ button next to a value adds one at the current time.**

In the terminal: the `keyframe` action (value pointer, instant and value). See [actions](../actions.md).

## How to move the playhead and play

Where it is: the top bar of the time strip.

Steps:

1. Click **Play** (it changes to **Pause** while it plays) to see the animation in the preview.
2. Drag the slider bar to go to an instant. To its left you see the current instant, for example “1.25 s”.
3. When you click a key in the strip, the playhead jumps to its instant.

While you play, the playhead advances by itself and returns to the start on reaching the end of the strip's duration.

In the terminal: the playhead position belongs only to the editor's preview; it is not an action. To see a particular instant use the capture in [cli](../cli.md).

## How to see and move keys in the time strip

Where it is: the time strip, one row for each animated value. Each row says “layer · value” and has a selector with the clock it follows. Keys are **◆** diamonds on a bar that goes from 0 to the duration.

Steps:

1. Click a diamond to choose the key. It is painted another color and the playhead jumps to its instant.
2. Drag it left or right to change its instant. While you drag, the value is shown in seconds (“1.25 s”). When you release, it is a single undo step.
3. To remove it, with the key chosen click **Remove the keyframe** in the top bar.

In the terminal: the `keyframe` action with `from` to move a key and `remove` to remove it. See [actions](../actions.md).

## How to change the smoothness of a span (the curve between two keys)

Where it is: the top bar of the time strip, with a key chosen. The bar's selector chooses how the value goes from that key to the next one.

Steps:

1. Choose a key in the strip.
2. In the bar's selector, choose the shape of the span. The options are **Linear**, **Jump** (the value changes abruptly at the next key) and, for each curve family, **Ease in**, **Ease out** and **Ease in and out**. The families are: quadratic, cubic, quartic, quintic, sine, exponential, circular, overshoot, bouncing and elastic. For example “Ease in (quadratic)”.
3. To draw the curve by hand, click **Curve and base of the keyframe** (curve icon). **Curve to the next keyframe** opens with two handles. Drag them: on a number the height is the value; on other things (lists of numbers and colors) the height is the share of the way, from 0 to 1.
4. If the key has no other after it, the dialog warns: the handles only curve the span between two keys of numbers, lists of numbers or colors. Choose one that has another after it.
5. **Straighten the curve** (it appears when the curve has handles of its own) returns the span to the shape of the selector.

Each handle drag is a single undo step.

In the terminal: the `keyframe` action with `ease`, `in_handle`, `out_handle` or `straighten`. See [actions](../actions.md).

## How to make keys relative (base value)

It makes keys add to a base value instead of being absolute values. That way you can change the layer's base value without redoing the animation.

Where it is: the same **Curve to the next keyframe** dialog, below the handles' drawing.

Steps:

1. Choose a key and click **Curve and base of the keyframe**.
2. Type the base value in the field (“A number, [x, y] or a color”) and click **Set the base**. From then on the text says: **The keyframes are added to this base value:**.
3. To go back to absolute keys, click **Absolute**.

In the terminal: the `keyframe` action with `base` or `absolute`.

## How to create a timeline

Where it is: the **Timelines** button (stopwatch icon), on the right of the time strip's top bar.

Steps:

1. Click **Timelines** and choose **Add a timeline**. One is created named “timeline_1” (or the next free one), 5 seconds long and repeating.
2. In the same menu each timeline appears with its name and its duration. Inside each one you have **Edit the timeline and its events…**, **Show in the inspector**, **Rename…** and **Remove; the values it drives follow the scene clock**.
3. For a value to follow the timeline, in its row of the time strip open the clock selector and choose the timeline. The first item, **Scene clock**, returns it to the general clock.
4. To change the name: **Rename…**. Only lowercase letters, digits and underscore are valid (up to 32 characters).

In the terminal: the `timeline` action with `add`, `to` (new name) or `remove`; the duration with `duration`. For a value to follow the timeline, put `timeline` in the `keyframe` action, or use `set` on the value's pointer. See [actions](../actions.md).
