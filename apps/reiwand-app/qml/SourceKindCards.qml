import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

FocusScope {
    id: cards

    // lower cards and tighter gaps, for when the full stack does not fit (natural is its full height)
    property bool compact: false
    property real natural: 0
    property var groups: [{
        "text": App.tr("With one click, no account"),
        "kinds": SourceKinds.catalog.filter(k => k.quick)
    }, {
        "text": App.tr("With your search or your server"),
        "kinds": SourceKinds.catalog.filter(k => !k.quick).concat(SourceKinds.plugged)
    }]
    readonly property var flat: groups.reduce((all, g) => all.concat(g.kinds), [])
    readonly property var bases: groups.map((g, i) => groups.slice(0, i).reduce((n, o) => n + o.kinds.length, 0))
    property int at: 0
    property Item word: null
    readonly property bool keyTarget: true

    signal picked(string kind)
    signal closed

    function enter() {
        if (word)
            word.focus = false;
        at = Math.max(0, at);
        forceActiveFocus(Qt.TabFocusReason);
    }

    function vertical(dir) {
        const g = bases.reduce((found, b, k) => at >= b ? k : found, 0);
        const i = at - bases[g];
        if (i + 2 * dir >= 0 && i + 2 * dir < groups[g].kinds.length)
            return at = at + 2 * dir;
        const o = g + dir;
        if (dir < 0 && g === groups.length - 1 && word && i < 2) {
            at = -1;
            return word.forceActiveFocus(Qt.TabFocusReason);
        }
        if (o < 0 || o >= groups.length)
            return;
        const size = groups[o].kinds.length;
        const row = dir > 0 ? 0 : size - 1 - (size - 1) % 2;
        at = bases[o] + Math.min(row + i % 2, size - 1);
    }

    implicitHeight: column.implicitHeight
    onImplicitHeightChanged: if (!compact)
        natural = implicitHeight
    implicitWidth: Look.kindsWidth
    Keys.onEscapePressed: closed()
    Keys.onLeftPressed: at = Math.max(0, at - 1)
    Keys.onRightPressed: at = Math.min(flat.length - 1, at + 1)
    Keys.onUpPressed: vertical(-1)
    Keys.onDownPressed: vertical(1)
    Keys.onReturnPressed: picked(flat[at].kind)
    Keys.onEnterPressed: picked(flat[at].kind)

    Column {
        id: column

        width: parent.width
        spacing: cards.compact ? Look.e2 : Look.e4

        Repeater {
            model: cards.groups

            delegate: Column {
                id: group

                required property var modelData
                required property int index
                readonly property int base: cards.bases[index]

                width: column.width
                spacing: Look.e2

                Item {
                    width: parent.width
                    height: heading.implicitHeight

                    Section {
                        id: heading

                        text: group.modelData.text
                    }

                    QQC2.Button {
                        id: plugins

                        objectName: "pluginsWord"
                        visible: group.index === cards.groups.length - 1
                        anchors.right: parent.right
                        y: heading.height - (heading.contentHeight + height) / 2
                        flat: true
                        text: App.tr("Provider plugins…")
                        icon.name: "plugs-connected"
                        Keys.onDownPressed: {
                            cards.at = group.base;
                            focus = false;
                            cards.forceActiveFocus(Qt.TabFocusReason);
                        }
                        Keys.onUpPressed: {
                            cards.at = Math.max(0, group.base - 1);
                            focus = false;
                            cards.forceActiveFocus(Qt.TabFocusReason);
                        }
                        Keys.onEscapePressed: cards.closed()
                        Keys.onReturnPressed: clicked()
                        Keys.onEnterPressed: clicked()
                        onClicked: cards.picked("plugins")
                        Component.onCompleted: if (visible)
                            cards.word = plugins
                    }
                }

                Flow {
                    width: parent.width
                    spacing: cards.compact ? Look.e2 : Look.e3

                    Repeater {
                        model: group.modelData.kinds

                        delegate: Rectangle {
                            id: kindCard

                            required property var modelData
                            required property int index
                            readonly property bool lit: cards.activeFocus && cards.at === group.base + index

                            width: (column.width - Look.e3) / 2
                            height: cards.compact ? Look.rowHeight - Look.e3 : Look.rowHeight
                            radius: Look.radiusSmall
                            color: lit ? Look.frost : Look.nightHigh
                            border.width: 1
                            border.color: lit ? Look.frost : Look.nightLine

                            Row {
                                anchors.fill: parent
                                anchors.margins: Look.e3
                                spacing: Look.e3

                                IconImage {
                                    anchors.verticalCenter: parent.verticalCenter
                                    name: kindCard.modelData.icon
                                    sourceSize: Qt.size(Look.kindIcon, Look.kindIcon)
                                    color: kindCard.lit ? Look.night : Look.moon
                                }

                                Column {
                                    anchors.verticalCenter: parent.verticalCenter

                                    Text {
                                        text: kindCard.modelData.line
                                        font.family: Look.textFont
                                        font.pixelSize: Look.tBody
                                        font.weight: Font.Medium
                                        color: kindCard.lit ? Look.night : Look.frost
                                    }

                                    Text {
                                        text: kindCard.modelData.name
                                        font.family: Look.textFont
                                        font.pixelSize: Look.tFoot
                                        color: kindCard.lit ? Look.night : Look.mist
                                    }
                                }
                            }

                            MouseArea {
                                anchors.fill: parent
                                onClicked: {
                                    cards.at = group.base + kindCard.index;
                                    cards.picked(kindCard.modelData.kind);
                                }
                            }
                        }
                    }
                }
            }
        }

    }
}
