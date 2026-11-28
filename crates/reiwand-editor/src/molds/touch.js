import { input } from "reiwand";

export function update() {
    return input.cursor.inside ? 1 : 0;
}

export const position = () => [input.cursor.x, input.cursor.y];
export const pressed = () => input.cursor.left;
