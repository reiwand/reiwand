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
