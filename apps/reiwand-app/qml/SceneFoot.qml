import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Rectangle {
    id: foot

    required property var page
    readonly property bool keyed: (page.view.tracks ?? []).length > 0 || page.mesh.clip !== ""
    // Under the canvas the timeline folds to its bar while nothing is keyed, as a footer; anywhere
    // else it is a panel like the others and keeps its whole place.
    property bool footer: true
    property bool unfolded: false
    // On a narrow window the footer starts folded even with keys, to give the canvas its room.
    readonly property bool narrow: (page.layout?.width ?? DeskLook.folds) < DeskLook.folds
    readonly property bool folded: footer && (!keyed || narrow) && !unfolded
    readonly property alias times: times
    readonly property real wanted: folded && !passage.visible ? Look.canvasBarHeight + Look.line : Look.canvasFootHeight

    // Keys of the timeline, wherever the keyboard is in it: Space plays or pauses, Home and End go to
    // the start or the end, the arrows step a tenth of a second, a second with Shift.
    function step(event) {
        const length = page.view.length ?? 10, playing = page.editor.playing;
        const at = {
            [Qt.Key_Home]: 0,
            [Qt.Key_End]: length,
            [Qt.Key_Left]: page.clock - (event.modifiers & Qt.ShiftModifier ? 1 : 0.1),
            [Qt.Key_Right]: page.clock + (event.modifiers & Qt.ShiftModifier ? 1 : 0.1)
        }[event.key];
        event.accepted = at !== undefined || event.key === Qt.Key_Space;
        if (!event.accepted)
            return;
        page.clock = at === undefined ? page.clock : Math.max(0, Math.min(length, at));
        page.editor.seek(page.clock, event.key === Qt.Key_Space ? !playing : false);
    }

    readonly property bool keyTarget: true

    color: Look.night
    activeFocusOnTab: true
    Keys.onPressed: event => step(event)

    Rectangle {
        width: parent.width
        height: Look.line
        color: Look.nightLine
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.topMargin: Look.line
        spacing: 0

        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: Look.canvasBarHeight - Look.line
            Layout.leftMargin: Look.e3
            Layout.rightMargin: Look.e3
            spacing: Look.e2

            QQC2.ToolButton {
                visible: foot.footer && (!foot.keyed || foot.narrow)
                icon.name: foot.folded ? "caret-right" : "caret-down"
                text: App.tr("Timeline")
                checkable: true
                checked: foot.unfolded
                onToggled: foot.unfolded = checked
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: App.tr("Lasts %1 s · repeats", Number((foot.page.view.length ?? 10).toFixed(2)))
                color: Look.mist
                elide: Text.ElideRight
            }

            QQC2.ToolButton {
                id: change

                icon.name: "clock"
                text: App.tr("Change…")
                onClicked: lines.popup(change, 0, change.height)

                SceneTimelineMenu {
                    id: lines

                    timeline: ({
                            "page": foot.page
                        })
                }
            }

            SceneTimeHeadKeys {
                visible: !foot.folded && !times.clipped
                page: foot.page
                timeline: times.timeline
            }
        }

        SceneTimes {
            id: times

            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: !foot.folded
            page: foot.page
        }
    }

    SceneTransitionFoot {
        id: passage

        anchors.fill: parent
        page: foot.page
    }
}
