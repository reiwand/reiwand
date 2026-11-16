import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var host

    readonly property var mine: [{
            "folder": "/tmp/one",
            "title": "One",
            "kind": "scene",
            "version": "1.0.0",
            "edited": 1,
            "poster": "",
            "missing": false
        }, {
            "folder": "/tmp/two",
            "title": "Two",
            "kind": "scene",
            "version": "",
            "edited": 0,
            "poster": "",
            "missing": true
        }, {
            "folder": "/tmp/three",
            "title": "Three",
            "kind": "scene",
            "version": "",
            "edited": 2,
            "poster": "",
            "missing": false,
            "recent": false
        }]
    readonly property var examples: [{
            "name": "pond",
            "title": "Pond",
            "kind": "scene",
            "license": "CC0-1.0",
            "picture": ""
        }]

    function build(projects, scenes) {
        return Qt.createComponent("CreatePage.qml").createObject(host, {
                "fixed": true,
                "width": 2000,
                "projects": projects,
                "scenes": scenes
            });
    }

    function expect(ok, what) {
        if (!ok)
            throw new Error("create: " + what);
    }

    function recent(page) {
        page.menu.ask(page.focused, page.focused.view.currentItem);
        expect(page.menu.visible && page.menu.picks[0].kind === "open" && page.menu.picks[3].off === true, "a card of the row offers its actions but not forgetting a present project");
        page.menu.close();
        page.focused.currentIndex = 2;
        page.act(page.focused);
        expect(page.all && page.focused === page.every.grid && page.facts.allModel.length === 3, "All my projects opens the grid of every project");
        page.every.find.text = "tw";
        expect(page.facts.allModel.length === 1 && page.facts.allModel[0].name === "Two", "the search box filters the projects by name");
        page.menu.ask(page.every.grid, page.every.grid.view.currentItem);
        expect(page.menu.picks[3].off === false, "a card of the grid can be removed from the list");
        page.menu.close();
        page.every.grid.forceActiveFocus();
        App.backend.press(Qt.Key_Escape, 0);
        expect(!page.all && page.focused === page.recentRow && page.query === "", "Esc goes back from the grid to the row");
    }

    function builtIn() {
        const real = Qt.createComponent("CreatePage.qml").createObject(host, {
                "width": 2000
            });
        expect(real.scenes.length >= 6 && real.scenes.every(scene => scene.picture.startsWith("data:image/")), "the examples built into the app are listed with their pictures");
        real.facts.copyOf(real.facts.learnModel[0]);
        expect(real.copy.visible && real.copy.facts.id !== undefined, "a built-in example opens the step that edits a copy: " + real.copy.problem);
        real.copy.close();
        real.destroy();
    }

    function exercise() {
        const empty = build([], []);
        empty.reset();
        expect(!empty.learn, "an empty profile without examples starts on the starters");
        expect(empty.facts.sceneModel.length === 4, "the scene starters are four");
        expect(empty.facts.singleModel.length === 3, "the single-file starters are three");
        expect(empty.facts.moreModel.length === 2, "the extra starters are two");
        expect(empty.rows.extra.visible && empty.rows.extra.model.length === 2, "preset and resource pack are cards in sight");
        expect(empty.facts.sceneModel.filter(entry => entry.picture !== undefined).length === 3, "three scene starters show what they draw");
        const first = build([], examples);
        first.reset();
        expect(first.learn && first.atExamples, "a first visit with examples opens on the examples");
        expect(first.facts.learnModel.length === 1, "the examples are listed");
        builtIn();
        const tiles = empty.focused;
        tiles.currentIndex = 1;
        empty.foot.rename(true);
        expect(empty.changing && empty.foot.titleField.activeFocus, "Change… opens the name fields with the focus in the title");
        for (const wide of [true, false]) {
            empty.foot.width = wide ? 4000 : 300;
            empty.foot.decide();
            expect(empty.foot.stacked !== wide, wide ? "the name fields share the title row" : "the name fields wrap under the title when the row is too narrow");
        }
        App.backend.press(Qt.Key_Escape, 0);
        expect(!empty.changing && empty.focused === tiles && tiles.activeFocus && tiles.currentIndex === 1, "Esc in the name fields goes back to the same tile");
        expect(empty.where.indexOf("/") >= 0, "the caption names the folder path: " + empty.where);
        const used = build(mine, examples);
        used.reset();
        expect(!used.learn, "a profile with projects opens on them");
        expect(used.facts.recentModel.length === 3 && used.facts.recentModel[2].all === true, "the recent projects are listed, without the one left out, and the row ends with all of them");
        recent(used);
        expect(!used.missing, "the newest project is present");
        expect(used.named === App.tr("My scene %1", 4), "the default title counts the projects: " + used.named);
        used.destroy();
        first.destroy();
        empty.destroy();
    }
}
