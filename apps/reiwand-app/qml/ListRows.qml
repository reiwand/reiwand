import QtQuick
import io.github.reiwand.app

Column {
    width: parent ? Math.min(Look.rowsWidth, parent.width - Look.explainWidth - Look.explainGap) : Look.rowsWidth
    spacing: Look.e2
}
