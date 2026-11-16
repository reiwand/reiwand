import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}

    implicitHeight: filas.height + note.implicitHeight + Look.e4

    Filas {
        id: filas

        Fila {
            text: App.tr("Allow downloading the depth model")
            note: App.tr("Downloaded once, when a picture first needs it")
            kind: "llave"
            checked: part.options["models.allow_download"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: App.act("SetOption", ["models.allow_download", String(checked)])
        }
    }

    FillNote {
        id: note

        x: Look.filaLado
        y: filas.height + Look.e4
        width: filas.width - Look.filaLado * 2
    }
}
