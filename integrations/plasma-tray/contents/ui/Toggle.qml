import QtQuick
import QtQuick.Layouts
import "../look"

Option {
    id: toggle

    checkable: true
    Accessible.role: Accessible.CheckBox
    Accessible.checked: checked

    Rectangle {
        parent: toggle.contentItem
        Layout.preferredWidth: Look.switchWidth
        Layout.preferredHeight: Look.switchHeight
        radius: height / 2
        color: toggle.checked ? Look.frost : Look.nightLine

        Rectangle {
            x: toggle.checked ? parent.width - width - 3 : 3
            y: 3
            width: Look.switchDot
            height: Look.switchDot
            radius: width / 2
            color: toggle.checked ? Look.night : Look.mist
        }
    }
}
