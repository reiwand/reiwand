import { engine } from "reiwand";

export const spin = (degrees, { params }) => ((degrees ?? 0) + params.speed * engine.dt) % 360;
