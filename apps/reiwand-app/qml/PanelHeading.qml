import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// A heading inside a panel of the editor: on the workbench a section head is no bigger than the
// words around it and the panel's tab (13 px, semibold), below the window's title. It sets one
// font size (Kirigami's heading sets a point size, and a pixel size over it warns on every frame).
QQC2.Label {
    property int level: 1

    font.pixelSize: Look.desk ? Look.tSection : level <= 2 ? Look.tRow : Look.tName
    font.weight: Font.DemiBold
    Accessible.role: Accessible.Heading
}
