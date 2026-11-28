import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: column

    required property Item view
    // A scroll bar drawn over the content (some styles do) keeps the rows from running under it.
    // Its room stays whether it shows or not: following it would change the width, the wrapping
    // and so the height, and with it whether it shows.
    readonly property var bar: view.QQC2.ScrollBar.vertical
    readonly property real barRoom: bar ? Math.max(0, (view.leftPadding ?? 0) + view.availableWidth - bar.x) : 0

    x: Look.ringRoom
    y: Look.ringRoom
    width: view.availableWidth - 2 * Look.ringRoom - barRoom

    Binding {
        target: column.view
        property: "contentHeight"
        value: column.implicitHeight + 2 * Look.ringRoom
    }
}
