import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Curves panel: the curve editor of the line in view in the Timeline and, for what is chosen, the
// curves along the life of its particles, the light of a lamp against the distance or the smoothing of
// the chosen link (a bound field), the same control as in the Inspector.
Rectangle {
    id: curves

    required property var page
    required property var times
    property bool focused: false
    readonly property alias body: body
    readonly property string pointer: page.editor.selected
    readonly property var node: (page.view.layers ?? []).find(item => item.pointer === pointer) ?? null
    readonly property string system: {
        const own = pointer.match(/^\/scene\/particles\/([^\/]+)/);
        if (own)
            return own[1].replace(/~1/g, "/").replace(/~0/g, "~");
        return node?.type === "particles" ? JSON.parse(page.editor.value(pointer + "/system") || "\"\"") : "";
    }
    readonly property string lamp: node?.type === "light" ? node.id : ""
    readonly property var link: (page.fields ?? []).find(field => field.key === "smooth" && field.widget === "smooth") ?? null
    readonly property string linkPointer: link?.pointer ?? ""
    readonly property var linkHost: link ? {
        "field": link,
        "page": page,
        "commit": (value, gesture) => page.change(link.pointer, value, gesture),
        "parse": (text, fallback) => {
            const typed = Number(String(text).replace(",", "."));
            return Number.isFinite(typed) ? typed : fallback;
        }
    } : null
    readonly property var modes: ["keys"].concat(system ? ["life"] : [], lamp ? ["falloff"] : [], link ? ["smooth"] : [])
    property string mode: "keys"
    readonly property string shown: modes.includes(mode) ? mode : "keys"

    color: Look.night
    clip: true
    // Choosing a particle system, a lamp or a link shows its curves; Keys is a click away.
    onSystemChanged: mode = system ? "life" : mode
    onLampChanged: mode = lamp ? "falloff" : mode
    onLinkPointerChanged: mode = linkPointer ? "smooth" : mode

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Look.e2
        spacing: Look.e2

        RowLayout {
            visible: curves.modes.length > 1
            spacing: Look.e1

            Repeater {
                model: curves.modes

                QQC2.ToolButton {
                    required property string modelData

                    text: ({
                            "keys": App.tr("Keys"),
                            "life": App.tr("Over life"),
                            "falloff": App.tr("Light falloff"),
                            "smooth": App.tr("Smoothing")
                        })[modelData]
                    checkable: true
                    checked: curves.shown === modelData
                    onClicked: curves.mode = modelData
                }
            }
        }

        SceneGraphBody {
            id: body

            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: curves.shown === "keys"
            page: curves.page
            sources: curves.times.sources
            now: curves.times.now
            live: curves.visible && visible
            keyed: curves.visible && visible && curves.focused
            explained: false
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: curves.shown === "life"
            contentWidth: availableWidth

            SceneCurvesLife {
                width: parent.width
                page: curves.page
                system: curves.shown === "life" ? curves.system : ""
            }
        }

        SceneCurvesFalloff {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: curves.shown === "falloff"
            page: curves.page
            lamp: curves.shown === "falloff" ? curves.lamp : ""
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: curves.shown === "smooth"
            contentWidth: availableWidth

            Loader {
                width: parent.width
                active: curves.shown === "smooth" && curves.linkHost !== null

                sourceComponent: SceneSmooth {
                    objectName: "curvesSmooth"
                    host: curves.linkHost
                }
            }
        }
    }
}
