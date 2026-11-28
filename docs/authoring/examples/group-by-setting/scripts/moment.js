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
