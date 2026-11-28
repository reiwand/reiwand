import QtQuick
import io.github.reiwand.app

Column {
    id: side

    required property Item row

    x: row.below ? Look.rowSide : row.width + Look.explainGap
    y: row.below ? row.labelEnd : row.base / 2 - 1.6 * Look.tExplain
    width: row.below ? row.width - Look.rowSide - Look.rowEnd : row.lit ? Math.max(Look.explainWidth / 2, Math.min(Look.explainWidth, row.room)) : Look.explainWidth
    spacing: Look.e3
    visible: row.lit && (row.explain !== "" || row.status !== "")

    Text {
        width: parent.width
        visible: side.row.explain !== ""
        text: side.row.explain
        wrapMode: Text.WordWrap
        lineHeight: 1.55
        font.family: Look.textFont
        font.pixelSize: Look.tExplain
        color: side.row.below ? side.row.ink : Look.frost
    }

    StatusLine {
        visible: side.row.status !== ""
        wrap: parent.width
        lit: side.row.below && !side.row.off
        text: side.row.status
        tone: side.row.statusTone
    }
}
