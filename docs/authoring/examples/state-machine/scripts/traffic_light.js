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
