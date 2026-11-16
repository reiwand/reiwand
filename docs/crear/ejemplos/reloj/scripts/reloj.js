import { scene, time } from "reiwand";

export function update() {
    scene.layer("hora").text = time.format("HH:mm");
    scene.layer("fecha").text = time.format("EEEE d 'de' MMMM");
}
