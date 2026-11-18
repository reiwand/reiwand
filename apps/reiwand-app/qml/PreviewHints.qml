import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Row {
    id: hints

    property string mode
    readonly property var model: mode === "vacio" ? [hint(["Intro"], App.tr("Open the library")), hint(["Esc"], App.tr("Close"))] : mode === "error" ? [hint(["Intro"], App.tr("Try again")), hint(["Esc"], App.tr("Close"))] : [hint(["←", "→"], App.tr("Move, or use the wheel")), hint(["Intro"], App.tr("Apply")), hint(["Esc"], App.tr("Close"))]
    readonly property string said: model.map(hint => hint.keys.map(k => k === "Intro" ? App.trc("key", "Enter") : k).join(" ") + " " + hint.text).join(", ")

    function hint(keys, text) {
        return {
            "keys": keys,
            "text": text
        };
    }

    spacing: Look.e5
    Accessible.role: Accessible.StaticText
    Accessible.name: said

    Repeater {
        model: hints.model

        delegate: Row {
            required property var modelData

            spacing: Look.e2

            Row {
                anchors.verticalCenter: parent.verticalCenter
                spacing: Look.e1

                Repeater {
                    model: modelData.keys

                    delegate: PreviewKey {
                        required property string modelData

                        key: modelData
                    }
                }
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: modelData.text
                color: Qt.alpha(Kirigami.Theme.textColor, 0.78)
                font.family: Look.letraTexto
                font.pixelSize: PreviewLook.tSub
            }
        }
    }
}
