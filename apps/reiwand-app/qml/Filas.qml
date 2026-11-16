import QtQuick
import io.github.reiwand.app

Column {
    width: parent ? Math.min(Look.filasAncho, parent.width - Look.explicaAncho - Look.explicaHueco) : Look.filasAncho
    spacing: Look.e2
}
