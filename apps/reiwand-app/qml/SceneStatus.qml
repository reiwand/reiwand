import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The status bar under the workbench: whether the scene is saved, its problems, how the preview is
// seen, the canvas size with the layout in use and, on the right, the keys of the focused panel.
Rectangle {
    id: status

    required property var page
    readonly property var area: page.docks
    readonly property int problems: SceneScriptFault.count(page)
    // A package that is not a scene has no canvas: its kind stands in for the canvas size.
    readonly property var size: area.stage?.canvas.size ?? null
    readonly property string focused: area.focusUnit
    readonly property var keys: keysOf(focused, area.curves?.shown ?? "keys")
    readonly property alias tones: tones

    // Curves has its point keys only while it shows keyframes; over life, falloff and smoothing it
    // has the keys of any panel.
    function keysOf(unit, curvesShow) {
        const tab = [["Tab", App.tr("next control")]];
        const own = {
            "canvas": status.size ? [["← → ↑ ↓", App.tr("move the layer")], ["Shift", App.tr("by 10 px")], ["Esc", App.tr("leave the canvas")]] : null,
            "layers": [["↑ ↓", App.tr("choose a layer")], ["Home End", App.tr("first or last")], ["Del", App.tr("remove")], ["Menu", App.tr("actions")]],
            "assets": [["↑ ↓", App.tr("choose an asset")], ["Home End", App.tr("first or last")]],
            "code": [["Ctrl+S", App.tr("save")], ["Ctrl+Tab", App.tr("other file")]],
            "timeline": [["Space", App.tr("play or pause")], ["Home End", App.tr("start or end")], ["← →", App.tr("step 0.1 s")], ["Shift", App.tr("by 1 s")]],
            "curves": curvesShow !== "keys" ? null : [["Ctrl+C Ctrl+V", App.tr("copy, paste points")], ["Del", App.tr("remove points")], ["F", App.tr("fit the chosen")], ["Home", App.tr("fit all")]]
        }[unit] ?? (unit ? tab : []);
        return own.concat(unit ? [["F6", App.tr("other panel")], ["F11", App.tr("enlarge")]] : [], [["Alt", App.tr("menus")]]);
    }

    implicitHeight: DeskLook.status
    color: Look.night

    // The whole editor, a scene's or a package's, is drawn at the workbench's density while it is
    // in view; publishing, drawn over it, keeps the app's sizes.
    Binding {
        target: Look
        property: "desk"
        value: status.page.visible && !(status.page.builder?.shown ?? false)
    }

    // Controls and labels with no size of their own follow the editor's density too.
    Binding {
        when: Look.desk && status.Window.window !== null
        target: status.Window.window
        property: "font.pixelSize"
        value: Look.tBody
    }

    SceneEditorTones {
        id: tones

        page: status.page
    }

    Rectangle {
        width: parent.width
        height: Look.line
        color: Look.nightLine
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: DeskLook.statusGap
        clip: true

        StatusLine {
            text: status.page.editor.dirty ? App.tr("Unsaved") : App.tr("Saved")
            tone: status.page.editor.dirty ? "" : "sprout"
            icon: status.page.editor.dirty ? "circle-half" : "check-circle-fill"
            size: DeskLook.statusIcon
            pixels: DeskLook.tStatus
            ink: Look.mist
        }

        Rectangle {
            implicitWidth: Look.line
            implicitHeight: DeskLook.sepHeight
            color: Look.nightLine
        }

        QQC2.AbstractButton {
            id: count

            implicitWidth: said.implicitWidth
            implicitHeight: DeskLook.status - 2 * Look.e1
            Accessible.name: said.text
            onClicked: status.page.layout.reveal("problems") || status.page.layout.reopen("problems")

            contentItem: StatusLine {
                id: said

                text: status.problems === 0 ? App.tr("No problems") : status.problems === 1 ? App.tr("1 problem") : App.tr("%1 problems", status.problems)
                tone: status.problems === 0 ? "sprout" : "amber"
                icon: status.problems === 0 ? "check-circle-fill" : "warning-fill"
                size: DeskLook.statusIcon
                pixels: DeskLook.tStatus
                ink: count.hovered ? Look.frost : Look.mist
            }
        }

        Rectangle {
            visible: status.size !== null
            implicitWidth: Look.line
            implicitHeight: DeskLook.sepHeight
            color: Look.nightLine
        }

        Text {
            visible: status.size !== null
            text: status.area.stage?.canvas.bar.seeAs.text ?? ""
            font.family: Look.textFont
            font.pixelSize: DeskLook.tStatus
            color: Look.mist
        }

        Rectangle {
            implicitWidth: Look.line
            implicitHeight: DeskLook.sepHeight
            color: Look.nightLine
        }

        Text {
            text: status.size ? App.tr("%1 × %2 · %3", status.size[0], status.size[1], status.page.layout.current.name) : App.tr("%1 · %2", LabelWords.values[status.page.kind] ?? status.page.kind, status.page.layout.current.name)
            font.family: Look.textFont
            font.pixelSize: DeskLook.tStatus
            color: Look.mist
        }

        Item {
            Layout.fillWidth: true
        }

        Repeater {
            model: status.keys

            delegate: Row {
                required property var modelData

                spacing: DeskLook.capSide

                Rectangle {
                    anchors.verticalCenter: parent.verticalCenter
                    width: Math.max(DeskLook.capWidth, cap.implicitWidth + 2 * DeskLook.capSide)
                    height: DeskLook.capHeight
                    radius: Look.radiusSmall
                    color: Look.nightHigh
                    border.width: Look.line
                    border.color: Look.nightLine

                    Text {
                        id: cap

                        anchors.centerIn: parent
                        text: parent.parent.modelData[0]
                        font.family: Look.textFont
                        font.pixelSize: DeskLook.tCap
                        font.weight: Font.DemiBold
                        color: Look.frost
                    }
                }

                Text {
                    anchors.verticalCenter: parent.verticalCenter
                    text: parent.modelData[1]
                    font.family: Look.textFont
                    font.pixelSize: DeskLook.tStatus
                    color: Look.mist
                }
            }
        }
    }
}
