# Complete examples

Each example is a whole scene, with its `manifest.json`, in `docs/authoring/examples/<name>/`.
The first six run in the repository tests for a few hundred frames, with simulated pointer, sound
and properties, and their results are checked. To see them on screen:

```
reiwand-ctl scene preview docs/authoring/examples/clock -o DP-1 --console
```

(Change `DP-1` to the name of your output, or remove it to use all of them.)

### 1. A clock

*Folder `clock`.* The shortest script: without declaration, it uses the reduced interface. It
writes the time and the date in two text layers, in the language of whoever uses the wallpaper.

**`scripts/clock.js`**

```js
import { scene, time } from "reiwand";

export function update() {
    scene.layer("time").text = time.format("HH:mm");
    scene.layer("date").text = time.format("EEEE d MMMM");
}
```

The declaration in `scene.json` is just `"scripts": [{ "path": "scripts/clock.js" }]`.

### 2. A layer that follows the pointer

*Folder `follow-pointer`.* Three circles chase the pointer, each at its own rush. The smooth
movement does not depend on the frame rate: the step is `1 - exp(-rush * dt)`. It declares
`uses: ["pointer"]` and the manifest declares `"inputs": ["pointer"]`.

**`scripts/follow.js`**

```js
import { scene, input, engine } from "reiwand";

const followers = [
    { layer: scene.layer("head"), rush: 8 },
    { layer: scene.layer("middle"), rush: 4 },
    { layer: scene.layer("tail"), rush: 2 },
];
const [cx, cy] = engine.canvas.map((v) => v / 2);
const spot = followers.map(() => [cx, cy]);

export function update() {
    const [px, py] = input.cursor.scene;
    followers.forEach(({ layer, rush }, i) => {
        const step = 1 - Math.exp(-rush * engine.dt);
        spot[i][0] += (px - spot[i][0]) * step;
        spot[i][1] += (py - spot[i][1]) * step;
        layer.position = spot[i];
    });
}
```

### 3. A reaction to sound

*Folder `sound-reaction`.* A ring grows with the bass (rises fast and falls slowly) and the screen
brightens a little on each beat. It declares `uses: ["audio"]` and `"inputs": ["audio"]`.

**`scripts/pulse.js`**

```js
import { scene, audio, engine } from "reiwand";

const pulse = scene.layer("pulse");
const flash = scene.layer("flash");
let size = 1;
let shine = 0;

export function update() {
    const goal = 1 + audio.bass * 0.8;
    const speed = goal > size ? 30 : 4;
    size += (goal - size) * (1 - Math.exp(-speed * engine.dt));
    pulse.scale = size;
    pulse.opacity = 0.4 + audio.level * 0.6;

    shine = audio.beat ? 0.1 : Math.max(0, shine - engine.dt);
    flash.opacity = shine;
}
```

### 4. A property that changes a group of layers

*Folder `group-by-setting`.* A `moment` property (automatic, day or night) shows one of two groups
of layers. In automatic, it checks the time of day every minute. It uses `properties()` to learn
of the change and `setInterval` (declare `timers`).

**`scripts/moment.js`**

```js
import { scene, props, engine, setInterval } from "reiwand";

function chosen() {
    const setting = props.get("moment");
    if (setting !== "auto") return setting;
    return engine.daytime >= 0.25 && engine.daytime < 0.75 ? "day" : "night";
}

function show() {
    const moment = chosen();
    for (const group of ["day", "night"]) {
        scene.layer(group).visible = group === moment;
    }
}

export function properties() {
    show();
}

setInterval(show, 60000);
```

### 5. A random choice at the start

*Folder `random-at-start`.* On start it picks one of three shapes and rotates it. With
`"seed": null` in `clock`, each start gives a different one.

**`scripts/random.js`**

```js
import { scene } from "reiwand";

const shapes = ["circle", "square", "ring"];

export function init() {
    const picked = shapes[Math.floor(Math.random() * shapes.length)];
    for (const id of shapes) {
        scene.layer(id).visible = id === picked;
    }
    scene.layer(picked).rotation = Math.random() * 360;
    console.log("picked shape:", picked);
}
```

### 6. A state machine

*Folder `state-machine`.* A traffic light. The `states` table says how long each state lasts and
which one follows; `enter` changes state and `update` checks whether the time has passed.

**`scripts/traffic_light.js`**

```js
import { scene, engine, console } from "reiwand";

const states = {
    green: { lasts: 4, next: "amber" },
    amber: { lasts: 1.5, next: "red" },
    red: { lasts: 4, next: "green" },
};
let current = "";
let since = 0;

function enter(name) {
    current = name;
    since = engine.time;
    for (const light of Object.keys(states)) {
        scene.layer(light).opacity = light === name ? 1 : 0.12;
    }
    console.log("state:", name);
}

export function init() {
    enter("red");
}

export function update() {
    if (engine.time - since >= states[current].lasts) {
        enter(states[current].next);
    }
}
```

### 7. One module for many values

*Folder `spin-by-value`.* A single module spins two layers at different speeds: each `rotation`
of `scene.json` is tied with `{ "script", "fn", "params" }`.

**`scripts/spin.js`**

```js
import { engine } from "reiwand";

export const spin = (degrees, { params }) => ((degrees ?? 0) + params.speed * engine.dt) % 360;
```

**In `scene.json`**

```json
"scripts": [{ "path": "scripts/spin.js", "events": ["update"], "uses": [],
              "params": { "speed": { "type": "number", "default": 30, "min": -720, "max": 720 } } }]
```

The `blade_slow` layer spins at the speed from `params` and `blade_fast` changes its own:

```json
"transform": { "rotation": { "script": "scripts/spin.js", "fn": "spin", "invert": false },
               "scale": [1.0, 1.0], "skew": [0.0, 0.0], "origin": "anchor" }
```

```json
"transform": { "rotation": { "script": "scripts/spin.js", "fn": "spin", "invert": false,
                             "params": { "speed": -150 } },
               "scale": [1.0, 1.0], "skew": [0.0, 0.0], "origin": "anchor" }
```
