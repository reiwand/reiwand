import { storage } from "reiwand";

let runs = 0;

export function init() {
    runs = (storage.get("runs") ?? 0) + 1;
    storage.set("runs", runs);
}

export const launches = () => runs;
