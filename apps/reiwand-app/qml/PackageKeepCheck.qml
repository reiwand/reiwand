import QtQuick
import io.github.reiwand.app

// A package keeps its arrangement in a file of its kind: moving one of its panels is read back
// there, and the scene's layouts are left as they were.
QtObject {
    id: check

    required property var page
    readonly property Layouts scene: Layouts {}
    readonly property Layouts mine: Layouts {
        place: check.page.kind
    }

    function exercise() {
        const layout = page.layout;
        const kind = page.kind;
        if (layout.desks.place !== kind || !layout.desks.file().endsWith("workbench-" + kind + ".json") || layout.desks.file() === scene.file())
            throw new Error(kind + ": the package saves its layouts in " + layout.desks.file());
        const before = scene.load();
        const side = ["right", "left", "down", "up"].find(at => layout.canMove("try", at));
        if (!side || !layout.move("try", side))
            throw new Error(kind + ": Try cannot be moved in " + layout.root);
        layout.flush();
        const read = JSON.parse(mine.load());
        const kept = read.desk ? JSON.stringify(JSON.parse(layout.desks.root(read.desk))) : "";
        if (read.refused !== "" || kept !== JSON.stringify(JSON.parse(layout.root)))
            throw new Error(kind + ": the arrangement read back is " + (read.refused || kept));
        if (scene.load() !== before)
            throw new Error(kind + ": saving the package arrangement changed the scene's layouts");
        layout.reset();
    }
}
