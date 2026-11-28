import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

// One animated value in the timeline: its name, the line that drives it and its lane of keys.
RowLayout {
    id: track

    required property var timeline
    required property var menu

    required property var modelData

    width: ListView.view.width
    height: Kirigami.Units.gridUnit * 1.6
    spacing: Look.e2

    QQC2.Label {
        Layout.preferredWidth: track.timeline.nameWidth
        leftPadding: Look.e4
        text: SceneWords.label(track.modelData.key)
        elide: Text.ElideRight

        TapHandler {
            acceptedButtons: Qt.RightButton
            onTapped: eventPoint => track.menu.onTrack(track.modelData, parent, eventPoint.position.x, eventPoint.position.y)
        }
    }

    QQC2.ComboBox {
        Layout.preferredWidth: track.timeline.lineWidth
        model: track.timeline.names.map(name => name || App.tr("Scene clock"))
        currentIndex: Math.max(0, track.timeline.names.indexOf(track.modelData.timeline ?? ""))
        onActivated: index => track.timeline.drive(track.modelData, index)
    }

    Rectangle {
        id: lane

        Layout.fillWidth: true
        Layout.fillHeight: true
        color: Kirigami.Theme.alternateBackgroundColor

        Rectangle {
            x: track.timeline.page.clock / track.timeline.length * lane.width
            width: 1
            height: parent.height
            color: Look.coral
        }

        SceneRepeater {
            id: listed

            rows: track.modelData.keys

            delegate: QQC2.Label {
                id: key

                readonly property var modelData: listed.rows[index]
                required property int index
                property real moved: -1

                x: (moved >= 0 ? moved : modelData[0]) / track.timeline.length * lane.width - width / 2
                anchors.verticalCenter: parent.verticalCenter
                text: "◆"
                color: track.timeline.chosen?.pointer === track.modelData.pointer && track.timeline.chosen?.index === index ? Kirigami.Theme.highlightColor : Kirigami.Theme.textColor

                QQC2.ToolTip.visible: key.moved >= 0
                QQC2.ToolTip.text: App.tr("%1 s", key.moved.toFixed(2))

                DragHandler {
                    target: null
                    yAxis.enabled: false
                    onActiveChanged: {
                        if (!active && key.moved >= 0 && key.moved !== key.modelData[0])
                            track.timeline.page.run("keyframe", {
                                "pointer": track.modelData.pointer,
                                "time": key.moved,
                                "from": key.modelData[0]
                            });
                        if (!active)
                            key.moved = -1;
                    }
                    onCentroidChanged: {
                        if (active)
                            key.moved = Math.round(Math.max(0, Math.min(1, key.mapToItem(lane, centroid.position.x, 0).x / lane.width)) * track.timeline.length * 100) / 100;
                    }
                }

                TapHandler {
                    acceptedButtons: Qt.RightButton
                    onTapped: eventPoint => track.menu.onKey(track.modelData, parent.index, parent, eventPoint.position.x, eventPoint.position.y)
                }

                TapHandler {
                    onTapped: {
                        track.timeline.chosen = {
                            "pointer": track.modelData.pointer,
                            "index": parent.index
                        };
                        track.timeline.seek(parent.modelData[0], false);
                    }
                }
            }
        }
    }
}
