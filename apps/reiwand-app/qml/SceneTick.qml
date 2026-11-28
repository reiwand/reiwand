import QtQuick
import QtQuick.Controls as QQC2

QQC2.CheckBox {
    id: tick

    contentItem: QQC2.Label {
        leftPadding: tick.indicator.width + tick.spacing
        text: tick.text
        wrapMode: Text.Wrap
        verticalAlignment: Text.AlignVCenter
    }
}
