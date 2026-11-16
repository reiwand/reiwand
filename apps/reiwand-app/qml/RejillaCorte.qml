import QtQuick
import io.github.reiwand.app

Item {
    id: corte

    required property int lead
    required property int columns
    required property real cellWidth
    required property real cellHeight
    required property real span
    readonly property int rows: Math.floor(lead / Math.max(1, columns))
    readonly property int rest: lead % Math.max(1, columns)
    readonly property real side: rest * cellWidth - Look.huecoEstante / 2

    visible: lead > 0
    Accessible.ignored: true

    Rectangle {
        visible: corte.rows > 0
        x: corte.rest > 0 ? corte.side : 0
        y: corte.rows * corte.cellHeight - Look.e5 / 2
        width: corte.span - x
        height: Look.linea
        color: Look.bruma
    }

    Rectangle {
        visible: corte.rest > 0
        x: corte.side
        y: corte.rows * corte.cellHeight - Look.e5 / 2
        width: Look.linea
        height: corte.cellHeight
        color: Look.bruma
    }

    Rectangle {
        visible: corte.rest > 0
        y: (corte.rows + 1) * corte.cellHeight - Look.e5 / 2
        width: corte.side
        height: Look.linea
        color: Look.bruma
    }
}
