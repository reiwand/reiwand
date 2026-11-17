import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Item {
    id: shots

    required property var preview
    readonly property var args: Qt.application.arguments
    readonly property string dir: argument("--shots")
    readonly property var pictures: argument("--pictures").split(",")
    readonly property string tag: argument("--tag")
    readonly property var titles: ["Harbour", "Aurora", "Waves", "Dunes", "Orbits", "Pond", "Snowfall", "Dusk", "Lights", "Pulse", "Solids", "Mist"]
    readonly property var kinds: ["image", "package", "video"]
    property int step: 0
    readonly property var plan: [["lleno-claro", false, 12, 2, false, false], ["lleno-oscuro", true, 12, 2, false, false], ["tres-claro", false, 3, 0, false, false], ["cargando-oscuro", true, 12, 4, true, false], ["cerca-oscuro", true, 12, 5, false, true], ["uno-oscuro", true, 1, 0, false, false], ["error-claro", false, 12, 2, false, false]]

    function argument(name) {
        const at = args.indexOf(name);
        return at >= 0 ? args[at + 1] : "";
    }

    function item(n, blank) {
        return {
            "uid": "uid" + n,
            "kind": kinds[n % 3] === "package" ? "package" : kinds[n % 3],
            "package": kinds[n % 3] === "package" ? {
                "kind": "scene"
            } : null,
            "title": titles[n % titles.length],
            "width": 3840,
            "height": 2160,
            "thumb": blank ? null : pictures[n % pictures.length]
        };
    }

    function state(count, at, blank) {
        const near = k => (at + k + count) % count;
        const list = k => [1, 2].filter(n => count > 2 * n || count > n + (k < 0 ? 0 : 1) && n === 1).map(n => item(near(k * n), blank));
        return {
            "output": preview.output,
            "active": true,
            "source": "playlist",
            "playlist_name": "Sea",
            "position": at,
            "count": count,
            "candidate": count > 0 ? item(at, blank) : null,
            "now": item(count > 1 ? near(-1) : at, blank),
            "ahead": count > 1 ? list(1) : [],
            "behind": count > 1 ? list(-1) : [],
            "plays": {
                "kind": "fade",
                "why": "general",
                "duration_ms": 600
            }
        };
    }

    function dress(dark) {
        const theme = preview.Kirigami.Theme;
        theme.backgroundColor = dark ? "#31363b" : "#eff0f1";
        theme.alternateBackgroundColor = dark ? "#3b4045" : "#e3e5e7";
        theme.textColor = dark ? "#fcfcfc" : "#232627";
        theme.highlightColor = "#3daee9";
    }

    function snap(name) {
        preview.contentItem.grabToImage(result => result.saveToFile(dir + "/" + name + "-" + tag + ".png"));
    }

    Component.onCompleted: {
        preview.width = Number(args[args.indexOf("--size") + 1]);
        preview.height = Number(args[args.indexOf("--size") + 2]);
        preview.show();
    }

    Timer {
        interval: 900
        repeat: true
        running: true
        onTriggered: {
            if (step % 2 === 1) {
                snap(plan[(step - 1) / 2][0]);
            } else if (step / 2 < plan.length) {
                const row = plan[step / 2];
                const name = row[0], dark = row[1], count = row[2], at = row[3], blank = row[4], hover = row[5];
                dress(dark);
                if (name.startsWith("error"))
                    preview.fail({
                        "message": "The service did not answer in time."
                    });
                else
                    preview.take(state(count, at, blank));
                if (hover)
                    Qt.callLater(() => preview.panel.reel.card(1).hovered = true);
            } else {
                Qt.exit(0);
            }
            step++;
        }
    }

    Image {
        parent: shots.preview.contentItem
        anchors.fill: parent
        z: -1
        source: App.url(shots.pictures[2])
        fillMode: Image.PreserveAspectCrop
    }
}
