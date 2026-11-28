import QtQuick
import io.github.reiwand.app

// The canvas bar names the handles of a chosen particle system and turns each kind off and on.
QtObject {
    required property var page

    function gather(item, out) {
        out.push(item);
        for (const child of item.children)
            gather(child, out);
        return out;
    }

    function exercise() {
        page.editor.select("/scene/particles/flakes");
        const all = gather(page, []);
        const bar = all.find(item => String(item).startsWith("SceneSwarmBar"));
        const words = all.filter(item => item.parent === bar && item.checkable).map(item => item.text);
        if (!bar?.visible || words.join() !== [App.tr("Emitter handles"), App.tr("Force arrows"), App.tr("Control points"), App.tr("Colliders")].join())
            throw new Error("particle handles: the canvas bar offers " + words);
        const walls = all.find(item => String(item).startsWith("SceneSwarmWalls"));
        SceneSwarmShow.flip("colliders");
        if (SceneSwarmShow.shown("colliders") || (walls && walls.visible))
            throw new Error("particle handles: hiding the colliders left them drawn");
        SceneSwarmShow.flip("colliders");
        if (!SceneSwarmShow.shown("colliders") || SceneSwarmShow.off.length !== 0)
            throw new Error("particle handles: the colliders did not come back");
    }
}
