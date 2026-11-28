import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

// One effect of a layer's stack: a row of the workbench's density with its name and, on the right,
// what happens to it on the device the preview is seen as (left out, frozen or lighter).
QQC2.ItemDelegate {
    id: row

    required property var page
    required property var effect
    property bool lit: false
    readonly property var level: {
        SceneLook.effective;
        const info = JSON.parse(page.editor.rig("", JSON.stringify({
            "effect_cost": {
                "effect": effect.effect ?? "",
                "optional": effect.optional ?? false
            }
        })) || "null");
        return (info?.levels ?? []).find(item => item.quality === SceneLook.effective) ?? null;
    }
    readonly property string note: {
        if (!level)
            return "";
        const device = SceneLook.names[Math.max(0, SceneLook.levels.indexOf(level.quality))];
        if (level.units === null)
            return level.form === "frozen" ? App.tr("%1: frozen", device) : App.tr("%1: left out", device);
        return level.form === "lighter" ? App.tr("%1: lighter", device) : "";
    }

    implicitHeight: DeskLook.row
    topPadding: 0
    bottomPadding: 0
    leftPadding: Look.e1
    rightPadding: Look.e2
    text: SceneWords.effect(effect.effect ?? "")
    Accessible.name: note ? text + ", " + note : text

    background: Rectangle {
        color: row.lit ? Look.frost : row.hovered ? Look.nightLine : "transparent"
        radius: Look.radius
    }

    contentItem: RowLayout {
        spacing: Look.e2

        IconImage {
            name: "sparkle"
            sourceSize: Qt.size(DeskLook.tabIcon, DeskLook.tabIcon)
            color: row.lit ? Look.night : Look.mist
        }

        QQC2.Label {
            Layout.fillWidth: true
            text: row.text
            color: row.lit ? Look.night : Look.frost
            font.pixelSize: DeskLook.tRow
            elide: Text.ElideRight
        }

        QQC2.Label {
            visible: row.note !== ""
            text: row.note
            color: row.lit ? Look.night : Look.mist
            font.pixelSize: DeskLook.tKind
        }
    }
}
