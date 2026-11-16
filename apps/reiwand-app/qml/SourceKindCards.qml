import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

FocusScope {
    id: cards

    property bool compact: false
    property var groups: [{
        "text": App.tr("With one click, no account"),
        "kinds": SourceKinds.catalog.filter(k => k.quick)
    }, {
        "text": App.tr("With your search or your server"),
        "kinds": SourceKinds.catalog.filter(k => !k.quick)
    }]
    readonly property var flat: groups.reduce((all, g) => all.concat(g.kinds), [])
    property int at: 0
    readonly property bool keyTarget: true

    signal picked(string kind)
    signal closed

    function enter() {
        forceActiveFocus(Qt.TabFocusReason);
    }

    function vertical(dir) {
        const split = groups[0].kinds.length;
        const g = at < split ? 0 : 1;
        const i = at - (g ? split : 0);
        if (i + 2 * dir >= 0 && i + 2 * dir < groups[g].kinds.length)
            return at = at + 2 * dir;
        const o = g + dir;
        if (o < 0 || o >= groups.length)
            return;
        const size = groups[o].kinds.length;
        const row = dir > 0 ? 0 : size - 1 - (size - 1) % 2;
        at = (o ? split : 0) + Math.min(row + i % 2, size - 1);
    }

    implicitHeight: column.implicitHeight
    implicitWidth: Look.tiposAncho
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
        spacing: Look.e4

        Repeater {
            model: cards.groups

            delegate: Column {
                id: group

                required property var modelData
                required property int index
                readonly property int base: index === 0 ? 0 : cards.groups[0].kinds.length

                width: column.width
                spacing: Look.e2

                Seccion {
                    text: group.modelData.text
                }

                Flow {
                    width: parent.width
                    spacing: Look.e3

                    Repeater {
                        model: group.modelData.kinds

                        delegate: Rectangle {
                            id: kindCard

                            required property var modelData
                            required property int index
                            readonly property bool lit: cards.activeFocus && cards.at === group.base + index

                            width: (column.width - Look.e3) / 2
                            height: Look.altoFila
                            radius: Look.radioChico
                            color: lit ? Look.escarcha : Look.nocheAlta
                            border.width: 1
                            border.color: lit ? Look.escarcha : Look.nocheLinea

                            Row {
                                anchors.fill: parent
                                anchors.margins: Look.e3
                                spacing: Look.e3

                                IconImage {
                                    anchors.verticalCenter: parent.verticalCenter
                                    name: kindCard.modelData.icon
                                    sourceSize: Qt.size(Look.tipoIcono, Look.tipoIcono)
                                    color: kindCard.lit ? Look.noche : Look.luna
                                }

                                Column {
                                    anchors.verticalCenter: parent.verticalCenter

                                    Text {
                                        text: kindCard.modelData.line
                                        font.family: Look.letraTexto
                                        font.pixelSize: Look.tCuerpo
                                        font.weight: Font.Medium
                                        color: kindCard.lit ? Look.noche : Look.escarcha
                                    }

                                    Text {
                                        text: kindCard.modelData.name
                                        font.family: Look.letraTexto
                                        font.pixelSize: Look.tPie
                                        color: kindCard.lit ? Look.noche : Look.bruma
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
