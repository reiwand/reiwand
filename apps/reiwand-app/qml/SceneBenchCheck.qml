import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function fail(what) {
        throw new Error("workbench: " + what);
    }

    function made() {
        return Qt.createComponent("SceneLayout.qml").createObject(page, {
            "width": 1280,
            "height": 720
        });
    }

    function refused(file, text, said) {
        App.writeText(file, text);
        const read = made();
        if (!read.held || !read.refused.includes(said) || read.desk !== read.desks.fresh())
            fail("a file that cannot be read was not refused with " + said + ": " + read.refused);
        read.move("assets", "right");
        read.flush();
        if (App.readText(file) !== text)
            fail("a refused file was written over");
        return read;
    }

    // A file with one broken layout: the others are kept, the notice names it, and it is written again.
    function mended(file, text, said) {
        App.writeText(file, text);
        const read = made();
        if (read.held || !read.refused.includes(said))
            fail("a file with a broken layout was not mended naming " + said + ": " + read.refused);
        read.move("assets", "right");
        read.flush();
        if (App.readText(file) === text)
            fail("a mended file was not written again");
        read.destroy();
    }

    function kits(layout) {
        for (let at = 0; at < layout.kits.length; at++) {
            if (!layout.chooseKey(at + 1) && at > 0)
                fail("Ctrl+" + (at + 1) + " did not choose a layout");
            if (layout.parsed.chosen.factory !== layout.kits[at] || layout.current.name !== BenchWords.layout(layout.kits[at]))
                fail("Ctrl+" + (at + 1) + " chose " + JSON.stringify(layout.parsed.chosen));
            if (layout.groups.length !== JSON.parse(layout.bench.blueprint(layout.root, 30, 20)).length || !layout.groupOf("canvas"))
                fail("the layout " + layout.kits[at] + " does not build its blueprint: " + layout.root);
        }
        layout.chooseKey(1);
    }

    function own(layout) {
        layout.move("assets", "right");
        const moved = layout.root;
        if (!layout.saveAs("Check layout") || layout.parsed.chosen.own !== "Check layout" || layout.root !== moved)
            fail("Save as new layout did not keep the workbench under its name");
        if (layout.saveAs("Check layout") || layout.desks.nameFault(layout.desk, "Check layout", -1) === "")
            fail("a second layout took a name already taken");
        layout.chooseKey(2);
        if (!layout.chooseKey(7) || layout.root !== moved)
            fail("Ctrl+7 did not choose the first layout of the author");
        layout.close("problems");
        layout.reset();
        if (layout.root !== moved)
            fail("resetting an own layout did not go back to how it was saved");
        if (!layout.rename(0, "Checked") || layout.current.name !== "Checked")
            fail("renaming the layout in use lost it");
        if (!layout.remove(0) || layout.parsed.own.length !== 0 || layout.parsed.chosen.factory !== "scene")
            fail("removing the layout in use did not go back to Scene");
        layout.reset();
    }

    function exercise() {
        const layout = page.layout;
        const file = layout.desks.file();
        layout.reset();
        kits(layout);
        own(layout);
        layout.move("assets", "right");
        layout.flush();
        const again = made();
        if (again.held || again.root !== layout.root || again.desk !== layout.desk)
            fail("saved layouts did not read back as they were: " + again.desk);
        again.destroy();
        const before = layout.closed;
        layout.close("timeline");
        layout.flush();
        const shut = made();
        const want = JSON.stringify(before.concat(["timeline"]).sort());
        if (shut.held || JSON.stringify(shut.closed.slice().sort()) !== want || JSON.stringify(JSON.parse(App.readText(file)).content.factory.scene.closed.slice().sort()) !== want)
            fail("a closed panel was not saved as closed: " + App.readText(file));
        shut.reopen("timeline");
        shut.destroy();
        const second = JSON.stringify({
            "format": 9,
            "content": {
                "root": JSON.parse(layout.usual),
                "closed": []
            }
        });
        const old = refused(file, second, "(format 9)");
        old.reset();
        if (old.held || old.refused !== "" || JSON.parse(App.readText(file)).format !== 8)
            fail("resetting the layout did not write the usual layouts");
        old.destroy();
        const cut = JSON.parse(layout.desks.fresh());
        const whole = {
            "format": 8,
            "content": cut
        };
        delete cut.factory.scene.root.split.nodes[0].group.shown;
        mended(file, JSON.stringify(whole), App.tr("Scene"));
        cut.factory.scene.root.split.nodes[0].group.shown = "canvas";
        mended(file, JSON.stringify(whole), App.tr("Scene"));
        cut.factory.scene.root.split.nodes[0].group.shown = "assets";
        cut.factory.particles.closed = ["layers"];
        mended(file, JSON.stringify(whole), App.tr("Particles"));
        layout.reset();
        if (JSON.parse(App.readText(file)).content.factory.scene.root.split.nodes.length !== 3)
            fail("the usual layouts were not saved again");
    }
}
