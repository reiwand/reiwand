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
