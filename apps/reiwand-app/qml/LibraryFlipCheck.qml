import QtQuick
import io.github.reiwand.app

Timer {
    id: check

    required property var host
    readonly property var page: host.pageStack.currentItem
    readonly property var expected: ({
            "0": 356,
            "1": 40,
            "2": 13,
            "5": 341,
            "6": 2
        })
    readonly property var pairs: [[5, 6], [6, 5], [0, 6], [1, 5], [2, 6], [0, 5]]
    readonly property var gaps: [0, 1, 3]
    property var plan: []
    property int wait: 0
    property int still: 0
    property int rounds: 0
    property var broken: []
    property bool finished: !App.backend.claim("library.flip")

    function must(ok, text) {
        if (!ok)
            throw new Error("library filters switched fast: " + text);
    }

    function round(first, second, count, gap) {
        const steps = [];
        for (let n = 0; n < count; n++)
            steps.push({
                "word": n % 2 === 0 ? first : second,
                "gap": gap
            });
        return steps;
    }

    function typed(gap) {
        return ["c", "cr", "crowd 3", "crowd 35", "crowd 3", "", "crowd 1", ""].map(text => ({
                    "text": "crowd:yes " + text,
                    "gap": gap
                }));
    }

    function begin() {
        if (finished)
            return;
        host.showPage(0);
        let all = [{
                "settle": true
            }];
        for (const gap of gaps)
            for (const pair of pairs)
                for (const count of [2, 3, 7])
                    all = all.concat(round(pair[0], pair[1], count, gap)).concat([
                        {
                            "audit": true
                        }
                    ]);
        for (const gap of gaps)
            all = all.concat(typed(gap)).concat([
                {
                    "audit": true
                }
            ]);
        plan = all;
        start();
    }

    function slots(list) {
        return Array.from(list.contentItem.children).filter(item => item.room !== undefined && item.ficha !== undefined && item.caption !== undefined);
    }

    function audit() {
        const bar = page.bar;
        const library = page.library;
        const list = page.covers.shelf.view;
        const word = bar.find.text === "crowd:yes " ? bar.word : -1;
        const all = slots(list);
        const owned = all.filter(item => item.index >= 0 && item.index < list.count && list.itemAtIndex(item.index) === item);
        const stray = all.filter(item => owned.indexOf(item) < 0 && item.visible);
        const lit = all.filter(item => item.ficha !== null && item.visible);
        const rows = list.count;
        const total = String(word) in expected ? expected[String(word)] : library.total;
        let ordered = true;
        for (let at = 1; at < list.count; at++) {
            const a = list.itemAtIndex(at - 1);
            const b = list.itemAtIndex(at);
            if (a && b && b.x <= a.x)
                ordered = false;
        }
        const foreign = word === 6 && [0, 1].some(at => !["00003", "00350"].includes(String(library.uid(at)).slice(-5)));
        const found = [library.total === total, list.count === rows, stray.length, lit.length <= 1, ordered, !foreign];
        if (found.join() !== "true,true,0,true,true,true")
            broken.push(["round " + rounds, "word " + word, "total " + library.total + "/" + total, "rows " + rows, "count " + list.count, "stray " + stray.map(item => item.index + "@" + Math.round(item.x)).join(" "), "lit " + lit.length, "ordered " + ordered, "foreign " + foreign].join(", "));
        rounds++;
    }

    function act(step) {
        const bar = page.bar;
        if (step.text !== undefined) {
            bar.find.text = step.text;
            if (step.gap === 0)
                bar.changed();
            return;
        }
        if (bar.find.text !== "crowd:yes ")
            bar.find.text = "crowd:yes ";
        bar.words.picked(bar.shown.indexOf(step.word));
    }

    function finish() {
        stop();
        finished = true;
        page.bar.clear();
        must(broken.length === 0, broken.length + " of " + rounds + " rounds left the shelf wrong; first: " + broken.slice(0, 4).join(" | "));
    }

    interval: 16
    repeat: true
    onTriggered: {
        if (!page || page.library === undefined || host.pageStack.currentItem !== page)
            return host.showPage(0);
        if (wait > 0)
            return wait--;
        if (plan.length === 0)
            return finish();
        const step = plan[0];
        if (step.audit || step.settle) {
            const busy = page.library.loading;
            still = busy ? 0 : still + 1;
            if (still < 6)
                return;
            still = 0;
            plan = plan.slice(1);
            return step.audit ? audit() : undefined;
        }
        plan = plan.slice(1);
        act(step);
        wait = step.gap;
        if (step.gap === 0)
            while (plan.length > 0 && plan[0].gap === 0 && !plan[0].audit) {
                act(plan[0]);
                plan = plan.slice(1);
            }
    }
}
