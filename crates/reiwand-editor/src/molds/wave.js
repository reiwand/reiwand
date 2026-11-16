import { engine } from "reiwand";

export const wave = (value, { params }) => Math.sin(engine.time * params.speed) * params.size;
