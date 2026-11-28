import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function fail(what) {
        throw new Error("frame: " + what + " " + page.editor.error);
    }

    function exercise() {
        const use = page.use;
        page.change("/scene/layers/0/opacity", 0.75);
        if (!page.editor.dirty || !use.store() || page.editor.dirty)
            fail("saving before use did not save the scene");
        if (use.store())
            fail("a saved scene was saved again");
        let picked = null;
        use.ask(null, true, output => picked = output);
        if (App.outputs.length < 2 && picked !== "")
            fail("one screen still asked which screen");
        const screens = Qt.createComponent("SceneWhere.qml").createObject(page);
        screens.choose(null, true, output => picked = output + "!");
        screens.accepted();
        if (picked !== "!" || screens.title === "")
            fail("the screen question did not answer its caller");
        screens.destroy();
        use.show("");
        if (use.viewing !== "on" || use.note.shown === "")
            fail("viewing as wallpaper did not raise its notice");
        use.stop();
        if (use.viewing !== "" || use.note.shown !== "")
            fail("stopping the view left its notice");
        use.ended();
        if (use.note.shown === "" || use.viewing !== "ended")
            fail("the end of the view was not told");
        use.back();
        if (use.note.shown !== "" || page.editor.live)
            fail("going back to the editor left the view on");
        for (const unit of page.layout.groupOf("assets").panels)
            page.layout.reveal(unit);
        if (!["assets", "scripts"].every(unit => page.layout.groupOf("assets").panels.includes(unit)))
            fail("the left side does not hold Assets and Scripts: " + page.layout.groupOf("assets").panels);
        page.layout.reveal("assets");
        if (typeof page.layers.add !== "function")
            fail("the empty scene cannot open the layer kinds");
    }
}
