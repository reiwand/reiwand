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
