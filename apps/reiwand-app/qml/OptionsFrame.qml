import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Item {
    id: frame

    required property var page
    required property int current
    property var sections: []
    readonly property var route: page?.route ?? []
    readonly property alias seat: seat
    readonly property point noticeAt: Qt.point(width - Look.e8, head.y + head.height + Look.e3)
    readonly property point hintsAt: Qt.point(panel.x + Look.e8, height - Look.e6)

    signal chosen(int page)
    signal escaped

    function climb(steps) {
        for (let i = 0; i < steps && frame.page && frame.page.routeBack; i++)
            frame.page.routeBack();
    }

    Keys.onEscapePressed: frame.escaped()

    Rectangle {
        width: Look.menuWidth
        height: frame.height
        gradient: Gradient {
            orientation: Gradient.Horizontal

            GradientStop {
                position: 0
                color: Qt.rgba(Look.night.r, Look.night.g, Look.night.b, 0.78)
            }

            GradientStop {
                position: 1
                color: Qt.rgba(Look.night.r, Look.night.g, Look.night.b, 0.55)
            }
        }
    }

    Rectangle {
        id: panel

        x: Look.menuWidth
        width: frame.width - x
        height: frame.height
        color: Look.night

        ColumnLayout {
            id: head

            x: Look.e8
            y: Look.marginTop + Look.e3
            width: panel.width - Look.e8 * 2
            spacing: Look.e5

            RowLayout {
                Layout.fillWidth: true
                spacing: Look.e5

                Loader {
                    active: !!(frame.page && frame.page.badge)
                    visible: active && !!item && item.implicitWidth > 0
                    sourceComponent: frame.page && frame.page.badge ? frame.page.badge : null
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    spacing: Look.e1

                    Text {
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                        text: frame.page?.title ?? ""
                        color: Look.frost
                        font.family: Look.titleFont
                        font.pixelSize: Look.tPage
                        font.weight: Font.Bold
                        font.letterSpacing: -0.03 * Look.tPage
                        Accessible.role: Accessible.Heading
                        Accessible.name: text
                    }

                    Text {
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                        visible: text !== ""
                        text: frame.page?.caption ?? ""
                        color: Look.mist
                        font.pixelSize: Look.tBody
                        textFormat: Text.PlainText
                    }
                }

                Kirigami.ActionToolBar {
                    Layout.fillWidth: true
                    alignment: Qt.AlignRight
                    actions: frame.page?.actions ?? []
                }

                Loader {
                    active: !!(frame.page && frame.page.head)
                    sourceComponent: frame.page && frame.page.head ? frame.page.head : null
                }
            }

            Row {
                visible: frame.sections.length > 1
                spacing: Look.e5

                Repeater {
                    model: frame.sections

                    delegate: SectionWord {
                        required property var modelData

                        text: modelData[1]
                        current: frame.current === modelData[0]
                        onClicked: frame.chosen(modelData[0])
                    }
                }
            }

            RowLayout {
                visible: frame.route.length > 0
                spacing: Look.e2

                Repeater {
                    model: frame.route

                    delegate: RowLayout {
                        id: step

                        required property string modelData
                        required property int index
                        readonly property bool last: index === frame.route.length - 1

                        spacing: Look.e2

                        Text {
                            Accessible.role: step.last ? Accessible.StaticText : Accessible.Link
                            Accessible.name: text
                            text: step.modelData
                            color: step.last ? Look.frost : Look.mist
                            font.family: Look.titleFont
                            font.pixelSize: Look.tSection
                            font.weight: step.last ? Font.Bold : Font.Normal

                            HoverHandler {
                                enabled: !step.last
                                cursorShape: Qt.PointingHandCursor
                            }

                            TapHandler {
                                enabled: !step.last
                                onTapped: frame.climb(frame.route.length - 1 - step.index)
                            }
                        }

                        Kirigami.Icon {
                            visible: !step.last
                            Layout.preferredWidth: Look.e4
                            Layout.preferredHeight: Look.e4
                            source: "caret-right"
                            color: Look.mistLow
                        }
                    }
                }

                Hints {
                    Layout.leftMargin: Look.e3
                    model: [{
                            "keys": ["Esc"],
                            "text": App.tr("back")
                        }]
                }
            }
        }

        Item {
            id: seat

            x: Look.e8
            y: head.y + head.height + Look.e5
            width: panel.width - Look.e8 * 2
            height: panel.height - y - Look.e6 - Look.shortcutsHeight - Look.e5
        }
    }
}
