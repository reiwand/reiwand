import QtQuick
import QtQuick.Layouts
import org.kde.plasma.extras as PlasmaExtras
import "../look"

PlasmaExtras.Representation {
    id: panel

    required property var daemon
    readonly property var current: daemon.current
    readonly property var names: Object.keys(current.outputs ?? {})
    readonly property bool single: names.length === 1
    property string view: "main"
    property string target: ""
    readonly property var own: rows.rowOf(single ? names[0] : target)
    readonly property bool shortcut: (current.options ?? {})["shortcuts.enabled"] === true
    readonly property string keys: String((current.options ?? {})["shortcuts.next"])
    readonly property var screen: (current.outputs ?? {})[names.length > 0 ? names[0] : ""] ?? ({})
    property var fixed: ({})

    function dressed(tones) {
        const set = tones ? {
            "night": tones.night,
            "nightHigh": tones.night_high,
            "nightLine": tones.night_line,
            "mist": tones.mist,
            "mistLow": tones.mist_low,
            "moon": tones.moon,
            "moonDeep": tones.moon_deep,
            "veil": tones.veil
        } : fixed;
        Look.tone(set);
    }

    Layout.minimumWidth: 320
    Layout.preferredWidth: 360
    Layout.minimumHeight: 200
    Layout.preferredHeight: Math.min(implicitHeight, 560)
    collapseMarginsHint: true
    Component.onCompleted: {
        fixed = {
            "night": Look.night,
            "nightHigh": Look.nightHigh,
            "nightLine": Look.nightLine,
            "mist": Look.mist,
            "mistLow": Look.mistLow,
            "moon": Look.moon,
            "moonDeep": Look.moonDeep,
            "veil": Look.veil
        };
        dressed(screen.tones ?? null);
    }
    onScreenChanged: dressed(screen.tones ?? null)
    onNamesChanged: {
        if (view === "screen" && !names.includes(target))
            view = "main";
    }

    background: Rectangle {
        color: Look.night
    }

    Choose {
        daemon: panel.daemon
    }

    contentItem: Flickable {
        id: scroll

        contentWidth: width
        contentHeight: list.implicitHeight
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        implicitHeight: list.implicitHeight

        ColumnLayout {
            id: list

            width: scroll.width
            spacing: Look.e3

            Head {
                Layout.fillWidth: true
                Layout.margins: Look.e4
                Layout.bottomMargin: 0
                visible: panel.view === "main" && !panel.single
                titled: !panel.single
                daemon: panel.daemon
            }

            Option {
                Layout.fillWidth: true
                visible: panel.view !== "main"
                glyph: "caret-left"
                text: panel.view === "screen" ? panel.daemon.screen(panel.target) : i18n("More")
                onClicked: panel.view = "main"
            }

            Ask {
                Layout.fillWidth: true
                Layout.margins: Look.e4
                Layout.topMargin: 0
                daemon: panel.daemon
                inputs: panel.daemon.inputs
            }

            Updated {
                Layout.fillWidth: true
                Layout.margins: Look.e4
                Layout.topMargin: 0
                daemon: panel.daemon
            }

            Notice {
                Layout.fillWidth: true
                Layout.margins: Look.e4
                daemon: panel.daemon
                screens: panel.names.length
            }

            Rows {
                id: rows

                Layout.fillWidth: true
                Layout.leftMargin: Look.e4
                Layout.rightMargin: Look.e4
                visible: panel.view === "main"
                daemon: panel.daemon
                names: panel.names
                onMore: name => {
                    panel.target = name;
                    panel.view = "screen";
                }
            }

            Head {
                Layout.fillWidth: true
                Layout.leftMargin: Look.e4
                Layout.rightMargin: Look.e4
                visible: panel.view === "main" && panel.single
                daemon: panel.daemon
            }

            ColumnLayout {
                Layout.fillWidth: true
                visible: panel.view === "main" && panel.names.length > 0
                spacing: 0

                Option {
                    Layout.fillWidth: true
                    glyph: "swap"
                    text: i18n("Change with preview…")
                    hint: panel.shortcut ? panel.keys.split("+").join(" ") : ""
                    onClicked: panel.daemon.browse("")
                }

                Option {
                    Layout.fillWidth: true
                    glyph: "sliders-horizontal"
                    text: panel.single ? i18n("More") : i18n("More · all screens")
                    more: true
                    onClicked: panel.view = "more"
                }
            }

            More {
                Layout.fillWidth: true
                visible: panel.view === "more"
                daemon: panel.daemon
            }

            Loader {
                Layout.fillWidth: true
                active: panel.own !== null && (panel.view === "screen" || panel.view === "more" && panel.single)
                visible: active

                sourceComponent: Own {
                    daemon: panel.daemon
                    row: panel.own
                }
            }

            Foot {
                Layout.fillWidth: true
                Layout.margins: Look.e4
                Layout.topMargin: 0
                visible: panel.view === "main" || panel.view === "more"
                daemon: panel.daemon
                note: panel.view === "main"
            }
        }
    }
}
