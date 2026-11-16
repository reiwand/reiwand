import { engine } from "reiwand";

export const gira = (grados, { params }) => ((grados ?? 0) + params.velocidad * engine.dt) % 360;
