import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Row {
    id: stars

    property int rating

    signal rated(int stars)

    Repeater {
        model: 5

        delegate: QQC2.ToolButton {
            required property int index

            icon.name: index < stars.rating ? "rating" : "rating-unrated"
            text: App.tr("Stars: %1", index + 1)
            display: QQC2.AbstractButton.IconOnly
            onClicked: stars.rated(index + 1 === stars.rating ? 0 : index + 1)
        }
    }
}
