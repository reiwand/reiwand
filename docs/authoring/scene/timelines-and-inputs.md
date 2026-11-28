# Timelines and inputs

On this page: timelines, inputs and the sound of the whole scene.

## Timelines

```json
  "timelines": [
    {
      "id": "jump", "duration": 3, "loop": "repeat", "autoplay": true, "delay": 0.0, "events": [],
      "fps": null, "frames": null, "speed": 1.0, "wrap": false
    }
  ]
```

Up to 64 per scene.

| Field | Prefill | What it does |
|-------|---------|--------------|
| `id` | | Its name. |
| `duration` | | Seconds. Instead of this it can count `frames` at `fps` (1 to 240): then `duration` is written `0`, and a timeline in seconds writes `frames` and `fps` as `null`. The keys stay in seconds. |
| `loop` | `once` | `once`, `repeat` or `pingpong`. |
| `speed` | 1 | -100 to 100. Below 0 it goes backwards. |
| `delay` | 0 | Seconds before starting. |
| `autoplay` | `true` | Whether it plays when the scene starts. It counts only without a `trigger`: `start` always plays from the start, and any other trigger waits for what it names. |
| `trigger` | | What starts it: `start`, `beat`, `prop:<id>` (a property, or a button), `event:<name>`, `click:<layer>`, `enter:<layer>`, `leave:<layer>` or `media:play`, `media:pause`, `media:track`, `media:cover`. |
| `wrap` | `false` | Only with `repeat`: blends the end with the start of the next loop. |
| `events` | | Up to 64 `{ "t": seconds, "name": "name" }` (`[a-z0-9_]{1,32}`). Each one fires once per loop, and they can start other timelines and sounds with `event:<name>`. |

`click:`, `enter:` and `leave:` work with `solid` layers (see [layers.md](../layers.md)). `click:`
needs the `clicks` input and the other two `pointer`.

## Inputs

Some scenes need data from the computer. Each one is an input that you declare in the manifest
(`"inputs": ["audio", "pointer"]`) and that the user must allow:

| Input | Used by | What the scene sees |
|-------|---------|---------------------|
| `audio` | `{ "audio": ... }`, effects with sound | The analysis of the sound. |
| `pointer` | `cursor`, `parallax`, `follow` of effects | Where the pointer is. |
| `clicks` | `click:` and `solid` | Clicks on `solid` layers. |
| `media` | `{ "media": ... }`, covers | The music player. |
| `network` | web pages | Outside layer scenes. |

You do not have to guess. If you use something and the input is missing, `reiwand-ctl pack check`
says, for example, `inputs: the scene uses pointer at /layers/0 of scene.json; add "pointer"`. To
add it:

```
reiwand-ctl scene act MY-FOLDER manifest '{"inputs":{"inputs":["pointer"],"add":true}}'
```

The scene is only redrawn when an input it uses changes. Keys never reach a scene, and clicks are
off on each screen until the user turns them on.

## Reaction to the sound of the whole scene

```json
  "audio": { "attack": 0.02, "release": 0.25, "gain_release": 5 }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `attack` | 0.02 s | 0.001 to 2 | How long a sound value takes to rise. |
| `release` | 0.2 s | 0.01 to 10 | How long it takes to fall. |
| `gain_release` | required (the editor proposes 5 s) | 0.05 to 10 | With the `relative` kind: in how much time the reference (the strongest recent band) falls toward the current level after a beat. 5 s keeps the reference almost fixed; 0.3 to 0.5 s makes the bands sink right after each beat and recover before the next (pumping effect). |
