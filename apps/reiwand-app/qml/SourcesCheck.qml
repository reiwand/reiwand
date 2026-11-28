import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property RowsCheck rows: RowsCheck {}

    function fail(what) {
        throw new Error("sources: " + what);
    }

    function detail(id) {
        page.current = id;
        page.open("detail");
        if (!page.shown || page.shown.provider.id !== id)
            fail("the details of " + id + " did not open");
        return page.shown;
    }

    function clearing() {
        const shown = detail("daily");
        shown.clearing.open();
        if (!shown.clearing.visible || shown.clearing.title !== App.tr("Delete the stored photos?"))
            fail("the stored photos are not deleted through their confirmation");
        shown.clearing.close();
    }

    function plugin() {
        const kept = page.providers;
        const daily = kept.find(p => p.id === "daily");
        const gallery = Object.assign({}, daily, {
            "id": "gallery",
            "kind": "plugin:example-gallery",
            "hosts": ["api.example.org", "images.example.org"],
            "options": {
                "query": "lakes",
                "per_page": "30",
                "videos": "false",
                "refresh": "1d",
                "cache_mb": "1024"
            }
        });
        page.apply(kept.concat([gallery]));
        if (!page.cards.card("gallery"))
            fail("a source of a provider plugin has no card");
        page.apply(kept.concat([Object.assign({}, gallery, {
                "state": "error",
                "last_error": "the plugin failed: Error: fetch failed\n    at fetch (eval_script:11:50)"
            })]));
        if (page.cards.card("gallery").problem !== "the plugin failed: Error: fetch failed")
            fail("a source card shows more than the first line of its error: " + page.cards.card("gallery").problem);
        detail("gallery");
        page.open("list");
        page.apply(kept);
    }

    function plugins() {
        const kinds = page.starters.flat.map(k => k.kind);
        const word = CheckKit.find(page.starters, item => item.objectName === "pluginsWord");
        if (!kinds.includes("plugin:gallery") || !word)
            fail("adding a source offers neither the installed plugins nor their management: " + kinds.join());
        word.clicked();
        if (page.view !== "plugins")
            fail("the provider plugins word does not open them");
        const shown = page.shown;
        if (!shown || shown.plugins.length !== 2 || page.leaf !== App.tr("Provider plugins"))
            fail("the provider plugins did not open with the installed ones");
        shown.offer = {
            "plugin": page.plugins[0],
            "installed": "1.1.0",
            "step": "older",
            "added_hosts": ["cdn.gallery.example.org"],
            "secret_changed": true,
            "token": "t"
        };
        shown.asking.open();
        const text = shown.asking.text;
        if (shown.asking.title !== App.tr("Update %1?", "Example gallery") || !text.includes("cdn.gallery.example.org") || !text.includes("API key"))
            fail("the consent does not name the update, the new places or the key: " + text);
        if (!text.includes(App.tr("It is older than the installed version %1: installing it goes back.", "1.1.0")))
            fail("the consent does not warn that the plugin goes back: " + text);
        shown.asking.close();
        shown.leaving = page.plugins[0];
        if (shown.removing.primary !== "")
            fail("a plugin that sources use can be removed");
        shown.leaving = page.plugins[1];
        if (shown.removing.primary !== App.tr("Remove"))
            fail("an unused plugin cannot be removed");
        page.routeBack();
        if (page.view !== "pick")
            fail("going back from the plugins does not return to adding a source");
    }

    function more() {
        detail("art");
        page.deeper("");
        const shown = page.shown;
        if (!shown || shown.level !== "" || !shown.rows.visible)
            fail("the more options of a search source did not open");
        if (shown.words("purity") !== App.tr("For everyone") || shown.words("categories").split(" · ").length !== 2)
            fail("the more options do not name what the source brings");
        page.deeper("purity");
        if (page.shown.level !== "purity" || page.shown.listed.rows.length !== 3)
            fail("the content level does not list its three choices");
        page.deeper("colors");
        page.routeBack();
        if (page.shown.level !== "size")
            fail("going back from the colors does not return to the size");
        page.routeBack();
        page.routeBack();
        if (page.view !== "detail")
            fail("going back from the more options does not return to the details");
    }

    function failing(offline) {
        const art = page.providers.find(p => p.id === "art");
        const broken = Object.assign({}, art, {
            "state": "error",
            "offline": offline,
            "last_error": offline ? "no connection: the server could not be reached" : "http request failed: HTTP status server error (500 Internal Server Error)"
        });
        page.apply(page.providers.map(p => p.id === "art" ? broken : p));
        const card = page.cards.card("art");
        if (page.cards.offline !== offline)
            fail(offline ? "a source without connection does not show the no connection notice" : "a source in error shows the no connection notice");
        if (!card || card.problem !== (offline ? "" : App.tr("%1 answered with error %2", SourceKinds.of(art.kind).name, "500")))
            fail(offline ? "a source without connection shows a problem of its own" : "a source in error does not say its problem");
    }

    function crowd() {
        const many = [];
        for (let n = 0; n < 4; n++)
            many.push(...page.providers.map(p => Object.assign({}, p, {
                    "id": p.id + n,
                    "state": "ok",
                    "offline": false,
                    "last_error": ""
                })));
        page.apply(many);
        rows.settle(page.cards);
        rows.settle(page.cards);
        lined();
    }

    function lined() {
        const list = page.cards;
        const says = text => item => rows.button(item) && item.text === text;
        rows.focused(list, says(App.tr("Add a source")), "sources");
        rows.focused(list, item => item.words !== undefined && item.keyed !== undefined, "sources");
        rows.focused(list, says(App.tr("Rotate these photos")), "sources");
        const found = rows.problems(list, "sources");
        page.load();
        if (found.length > 0)
            fail(found.join(" | "));
    }

    function empty() {
        const kept = page.providers;
        page.apply([]);
        page.open("list");
        const found = [page.background.visible, page.starters.visible];
        page.apply(kept);
        if (found.join() !== "false,true")
            fail("without sources the list hides the wallpaper behind the page background, or loses its starting cards: " + found.join());
    }

    function exercise() {
        empty();
        failing(false);
        failing(true);
        page.load();
        clearing();
        plugin();
        plugins();
        more();
        page.open("list");
        crowd();
        destroy();
    }
}
