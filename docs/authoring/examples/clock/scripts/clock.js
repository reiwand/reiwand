import { scene, time } from "reiwand";

export function update() {
    scene.layer("time").text = time.format("HH:mm");
    scene.layer("date").text = time.format("EEEE d MMMM");
}
