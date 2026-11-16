import QtQuick
import io.github.reiwand.app

Column {
    id: side

    required property Item fila

    x: fila.below ? Look.filaLado : fila.width + Look.explicaHueco
    y: fila.below ? fila.labelEnd : fila.base / 2 - 1.6 * Look.tExplica
    width: fila.below ? fila.width - Look.filaLado - Look.filaFin : fila.lit ? Math.max(Look.explicaAncho / 2, Math.min(Look.explicaAncho, fila.room)) : Look.explicaAncho
    spacing: Look.e3
    visible: fila.lit && (fila.explain !== "" || fila.status !== "")

    Text {
        width: parent.width
        visible: side.fila.explain !== ""
        text: side.fila.explain
        wrapMode: Text.WordWrap
        lineHeight: 1.55
        font.family: Look.letraTexto
        font.pixelSize: Look.tExplica
        color: side.fila.below ? side.fila.ink : Look.escarcha
    }

    Estado {
        visible: side.fila.status !== ""
        wrap: parent.width
        lit: side.fila.below && !side.fila.off
        text: side.fila.status
        tone: side.fila.statusTone
    }
}
