import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

QtObject {
    id: check

    required property var host
    readonly property var shell: host.shell
    readonly property var allowed: ["left", "right", "up", "down", "Intro", "Esc", "Tab"]
    readonly property Component blank: Component {
        Kirigami.Page {}
    }

    function fail(what) {
        throw new Error("shell: " + what);
    }

    function words() {
        return host.shell.menu.all.map(entry => entry[1]).join(",");
    }

    function menu() {
        const wanted = ["Library", "Explore", "Lists", "Sources", "Create", "Settings", "Account"].map(word => App.tr(word)).join(",");
        if (words() !== wanted)
            fail("the menu reads " + words());
        for (let page = 0; page < host.pages.length; page++) {
            host.showPage(page);
            const marked = shell.menu.all.map((entry, at) => shell.menu.word(at)).filter(word => word.current);
            if (marked.length !== 1 || marked[0].page !== shell.menu.owner(page))
                fail("page " + page + " marks " + marked.length + " menu words");
        }
    }

    function seat(page) {
        host.showPage(page);
        const at = shell.menu.word(0).mapToItem(null, 0, 0);
        return [Look.menuAncho, Look.tMenu, Look.margen, Look.margenArriba, at.x, at.y].join(",");
    }

    function steady() {
        host.showPage(0);
        shell.titleFrame.need = host.height + 1000;
        const first = seat(0);
        for (let page = 1; page < host.pages.length; page++) {
            const found = seat(page);
            if (shell.kind !== "work" && found !== first)
                fail("the left menu of page " + page + " is not where it is on page 0: " + found + " against " + first);
        }
        shell.titleFrame.known = 0;
        shell.titleFrame.needs = ({});
        shell.titleFrame.need = 0;
        host.showPage(0);
        if (host.minimumWidth < Kirigami.Units.gridUnit * 62 || host.minimumHeight < Kirigami.Units.gridUnit * 38)
            fail("the window may be smaller than its pages need");
    }

    function hints(what) {
        const keys = [].concat(...shell.hints.model.map(hint => hint.keys));
        const stray = keys.filter(key => !allowed.includes(key));
        if (stray.length > 0)
            fail(what + " hints show the keys " + stray.join(","));
        return keys;
    }

    function keyboard() {
        host.showPage(0);
        shell.menu.forceActiveFocus(Qt.TabFocusReason);
        const onMenu = hints("menu");
        if (!onMenu.includes("up") || !onMenu.includes("Intro"))
            fail("the menu hints miss the arrows or Intro: " + onMenu.join(","));
        host.pageStack.currentItem.forceActiveFocus(Qt.TabFocusReason);
        const onPage = hints("page");
        if (!onPage.includes("Esc"))
            fail("the page hints miss Esc: " + onPage.join(","));
        if (!shell.hints.visible)
            fail("the hints line is hidden on a title screen");
        const cap = Qt.createComponent("KeyCap.qml").createObject(check.host, {
            "key": "Intro"
        });
        const word = cap.shown;
        cap.destroy();
        if (word !== App.trc("key", "Enter"))
            fail("the Enter key reads " + word + " in the hints instead of " + App.trc("key", "Enter"));
    }

    function seated(kind, frame) {
        if (host.kind !== kind || !frame.visible)
            fail("the " + kind + " skeleton is not shown");
        if (host.pageStack.parent !== frame.seat)
            fail("the page does not sit in the " + kind + " skeleton");
        for (const other of [shell.titleFrame, shell.optionsFrame, shell.workFrame])
            if (other !== frame && other.visible)
                fail("two skeletons show at once");
    }

    function skeletons() {
        host.showPage(4);
        seated("title", shell.titleFrame);
        host.showPage(5);
        seated("options", shell.optionsFrame);
        if (shell.optionsFrame.sections.length !== 0 || shell.optionsFrame.page.words.length !== 3)
            fail("the settings do not show their three tabs");
        host.showPage(0);
        host.work(blank);
        seated("work", shell.workFrame);
        if (shell.hints.visible || shell.menu.visible && shell.menu.parent.visible)
            fail("the work skeleton shows the menu or the hints");
        if (shell.workFrame.backButton.text !== App.tr("Library"))
            fail("the way back reads " + shell.workFrame.backButton.text);
        host.showPassiveNotification("probe");
        if (shell.notices.text !== "probe")
            fail("a notice does not reach the notice place");
        shell.notices.hide();
        shell.workFrame.backButton.clicked();
        if (host.current !== 0)
            fail("the way back opened page " + host.current);
    }

    function tones() {
        const fixed = JSON.parse(Look.bridge.derive(""));
        const keys = ["noche", "nocheAlta", "nocheLinea", "bruma", "brumaBaja", "luna", "lunaHonda", "velo"];
        const lost = keys.filter(key => fixed[key] === undefined);
        if (lost.length > 0)
            fail("the derived tones miss " + lost.join(","));
    }

    function exercise() {
        menu();
        steady();
        keyboard();
        skeletons();
        tones();
        destroy();
    }
}
