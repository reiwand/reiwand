import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.ShadowedRectangle {
    id: panel

    required property var preview
    readonly property real unit: preview.unit
    readonly property bool working: preview.mode !== "vacio" && preview.mode !== "error"
    property alias reel: reel
    property alias hints: hints

    width: Math.round(unit * (PreviewLook.focoAncho + 2 * (PreviewLook.cercaAncho + PreviewLook.lejosAncho) + 4 * PreviewLook.hueco + 2 * PreviewLook.panelLado))
    height: Math.round(unit * (PreviewLook.panelArriba + PreviewLook.panelAbajo + 2 * PreviewLook.tNombre + 150)) + reel.implicitHeight
    radius: Math.round(PreviewLook.panelRadio * unit)
    color: Kirigami.Theme.backgroundColor
    border.width: 1
    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
    shadow.size: Math.round(40 * unit)
    shadow.color: Qt.rgba(0, 0, 0, 0.45)
    shadow.yOffset: Math.round(10 * unit)
    Accessible.role: Accessible.Dialog
    Accessible.name: preview.title

    Row {
        id: head

        x: Math.round(PreviewLook.panelLado * panel.unit)
        y: Math.round(PreviewLook.panelArriba * panel.unit)
        width: panel.width - 2 * x
        height: label.implicitHeight

        Text {
            id: label

            width: parent.width / 2
            text: panel.preview.heading
            elide: Text.ElideRight
            color: Kirigami.Theme.textColor
            font.family: Look.letraTexto
            font.pixelSize: PreviewLook.tTitulo
            font.weight: Font.DemiBold
        }

        Text {
            width: parent.width / 2
            text: panel.preview.origin
            horizontalAlignment: Text.AlignRight
            elide: Text.ElideLeft
            color: Qt.alpha(Kirigami.Theme.textColor, 0.78)
            font.family: Look.letraTexto
            font.pixelSize: PreviewLook.tTitulo
        }
    }

    PreviewReel {
        id: reel

        anchors.left: parent.left
        anchors.right: parent.right
        y: head.y + head.height + Math.round(26 * panel.unit)
        visible: panel.working
        unit: panel.unit
        aspect: panel.preview.aspect
        calm: panel.preview.calm
        nowUid: panel.preview.current ? panel.preview.current.uid : ""
        animated: panel.preview.animated
        onChosen: offset => panel.preview.choose(offset)
    }

    PreviewCaption {
        anchors.horizontalCenter: parent.horizontalCenter
        y: reel.y + reel.height + Math.round(26 * panel.unit)
        width: parent.width - 2 * PreviewLook.panelLado * panel.unit
        visible: panel.working
        title: panel.preview.candidate ? panel.preview.candidate.title : ""
        detail: panel.preview.position
    }

    PreviewEmpty {
        anchors.horizontalCenter: parent.horizontalCenter
        y: reel.y + Math.round((reel.height - height) / 2) + Math.round(30 * panel.unit)
        width: Math.round(panel.width * 0.6)
        visible: !panel.working
        icon: panel.preview.mode === "error" ? "dialog-error" : "folder-pictures"
        title: panel.preview.mode === "error" ? App.tr("The wallpaper service does not answer") : App.tr("This screen has only one wallpaper")
        note: panel.preview.mode === "error" ? panel.preview.failure : App.tr("Add more wallpapers to the library to switch between them.")
        action: panel.preview.mode === "error" ? App.tr("Try again") : App.tr("Open the library")
        onActivated: panel.preview.press(Qt.Key_Return)
    }

    PreviewHints {
        id: hints

        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottom: parent.bottom
        anchors.bottomMargin: Math.round(PreviewLook.panelAbajo * panel.unit)
        mode: panel.preview.mode
    }
}
