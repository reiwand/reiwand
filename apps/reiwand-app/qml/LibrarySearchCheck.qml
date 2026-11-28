import QtQuick
import io.github.reiwand.app

// Types a word on the shelf of All, adds a letter that leaves few or no results and deletes it
// again: the covers of the word must come back without moving the shelf.
Timer {
    id: check

    required property var host
    readonly property var page: host.pageStack.currentItem
    readonly property var words: [["crowd", "crowdx"], ["c", "cx"], ["crowd 3", "crowd 35"], ["crowd 1", "crowd 1x"]]
    readonly property var gaps: [0, 2, 20]
    property var plan: []
    property int wait: 0
    property int still: 0
    property int tries: 0
    property int rounds: 0
    property var broken: []
    property bool finished: !App.backend.claim("library.search")

    function must(ok, text) {
        if (!ok)
            throw new Error("library search typed and deleted: " + text);
    }

    function round(word, longer, gap) {
        return [
            {
                "text": "crowd:yes ",
                "gap": 0
            },
            {
                "settle": true,
                "covers": true,
                "quiet": true
            },
            {
                "text": "crowd:yes " + word,
                "gap": gap
            },
            {
                "settle": true,
                "covers": true,
                "quiet": true
            },
            {
                "text": "crowd:yes " + longer,
                "gap": gap
            },
            {
                "text": "crowd:yes " + word,
                "gap": gap
            },
            {
                "settle": true,
                "covers": true,
                "said": word + " after " + longer + ", gap " + gap
            }
        ];
    }

    function begin() {
        if (finished)
            return;
        host.showPage(0);
        let all = [
            {
                "settle": true
            }
        ];
        for (const gap of gaps)
            for (const pair of words)
                all = all.concat(round(pair[0], pair[1], gap));
        plan = all;
        start();
    }

    function slots(list) {
        return Array.from(list.contentItem.children).filter(item => item.room !== undefined && item.cardItem !== undefined && item.caption !== undefined);
    }

    function covered(step) {
        const list = page.covers.shelf.view;
        const seen = slots(list).filter(item => item.visible && item.index >= 0 && item.index < list.count && item.x + item.width > list.contentX && item.x < list.contentX + list.width);
        const bare = seen.filter(item => String(item.picture) === "");
        const ok = seen.length > 0 && bare.length === 0;
        if (step === undefined || step.quiet)
            return ok;
        if (!ok)
            broken.push(step.said + ": " + bare.length + " of " + seen.length + " shelf covers without a picture");
        rounds++;
        return ok;
    }

    function act(step) {
        const bar = page.bar;
        if (bar.word !== 0)
            bar.words.picked(bar.shown.indexOf(0));
        page.all = false;
        bar.find.text = step.text;
        if (step.gap === 0)
            bar.changed();
    }

    function finish() {
        stop();
        finished = true;
        page.bar.clear();
        must(broken.length === 0, broken.length + " of " + rounds + " rounds left the covers empty; first: " + broken.slice(0, 4).join(" | "));
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
        if (step.settle) {
            still = page.library.loading ? 0 : still + 1;
            if (still < 6)
                return;
            if (step.covers && !covered() && ++tries < 150)
                return;
            still = 0;
            tries = 0;
            plan = plan.slice(1);
            if (step.covers)
                covered(step);
            return;
        }
        plan = plan.slice(1);
        act(step);
        wait = step.gap;
    }
}
