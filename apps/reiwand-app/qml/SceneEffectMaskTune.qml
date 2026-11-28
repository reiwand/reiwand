import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: tune

    required property var panel
    readonly property var mask: panel.mask !== null && typeof panel.mask === "object" ? panel.mask : ({})
    readonly property var channels: ["luminance", "r", "g", "b", "a"]

    visible: panel.source !== ""

    // The rows keep the label column of the mask's own rows above, and nothing here asks for
    // more width than a narrow panel has.
    SceneValueRow {
        visible: tune.panel.source === "texture" || tune.panel.source === "layer"
        text: App.tr("Channel")

        QQC2.ComboBox {
            Layout.fillWidth: true
            Accessible.name: App.tr("Channel")
            model: [App.tr("Brightness"), App.tr("Red"), App.tr("Green"), App.tr("Blue"), App.tr("Alpha")]
            currentIndex: Math.max(0, tune.channels.indexOf(tune.mask.channel ?? (tune.panel.source === "texture" ? "luminance" : "a")))
            onActivated: index => tune.panel.put({
                    "channel": tune.channels[index]
                })
        }
    }

    SceneCheckLine {
        text: App.tr("Invert: apply the effect where the mask is empty")
        checked: tune.mask.invert === true
        onToggled: tune.panel.put({
            "invert": checked
        })
    }

    SceneValueRow {
        visible: tune.panel.kind === "polygon" || tune.panel.kind === "star"
        text: tune.panel.kind === "polygon" ? App.tr("Sides") : App.tr("Points")

        QQC2.SpinBox {
            from: 3
            to: 64
            editable: true
            value: tune.panel.kind === "polygon" ? (tune.panel.mask.shape.polygon.sides ?? 6) : tune.panel.kind === "star" ? (tune.panel.mask.shape.star.points ?? 5) : 3
            onValueModified: tune.panel.count(value)
        }
    }

    SceneValueRow {
        visible: tune.panel.source === "shape"
        text: App.tr("Soft edge")

        SceneSlider {
            Layout.fillWidth: true
            Layout.preferredWidth: Look.railWidth
            from: 0
            to: Math.max(200, feather.value)
            stepSize: 1
            value: tune.mask.feather ?? 0
            onMoved: tune.panel.put({
                "feather": Math.round(value)
            }, true)
        }

        QQC2.SpinBox {
            id: feather

            from: 0
            to: 10000
            editable: true
            value: tune.mask.feather ?? 0
            textFromValue: (v, locale) => App.tr("%1 px", v)
            valueFromText: (text, locale) => parseInt(text) || 0
            onValueModified: tune.panel.put({
                "feather": value
            })
        }
    }

    SceneValueRow {
        text: App.tr("Strength")

        SceneSlider {
            Layout.fillWidth: true
            Layout.preferredWidth: Look.railWidth
            from: 0
            to: 1
            stepSize: 0.01
            value: typeof tune.mask.strength === "number" ? tune.mask.strength : 1
            enabled: tune.mask.strength === undefined || typeof tune.mask.strength === "number"
            onMoved: tune.panel.put({
                "strength": Math.round(value * 100) / 100
            }, true)
        }
    }
}
