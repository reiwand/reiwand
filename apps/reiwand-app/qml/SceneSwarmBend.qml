import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: bend

    required property var light
    readonly property var info: light.info
    readonly property var bent: info?.refract ?? null
    readonly property var page: light.tools.page

    function put(args, gesture) {
        SceneSwarmGlow.run(page, "refract", args, gesture);
    }

    QQC2.CheckBox {
        text: App.tr("Bend what is behind the particles")
        checked: bend.bent !== null
        enabled: (bend.info?.pictures ?? []).length > 0
        onToggled: bend.put(checked ? {
            "normal": bend.info.normal ?? bend.info.pictures[0],
            "units": "size",
            "amount": 0.3,
            "color": "none"
        } : {
            "off": true
        })
    }

    SceneRowFit {
        visible: bend.bent !== null

        QQC2.ComboBox {
            Layout.fillWidth: true
            model: bend.info?.pictures ?? []
            currentIndex: Math.max(0, (bend.info?.pictures ?? []).indexOf(bend.bent?.normal ?? ""))
            onActivated: index => bend.put({
                    "normal": bend.info.pictures[index]
                })
        }

        QQC2.ComboBox {
            readonly property var units: ["size", "pixels"]

            model: [App.tr("Proportional to the size"), App.tr("In pixels")]
            currentIndex: bend.bent?.units === "pixels" ? 1 : 0
            onActivated: index => bend.put({
                    "units": units[index]
                })
        }

        SceneSlider {
            readonly property bool pixels: bend.bent?.units === "pixels"

            Layout.fillWidth: true
            from: 0
            to: pixels ? 4096 : 1
            value: pixels ? (bend.bent?.shift ?? App.fill("Action.particles.refract.shift")) : (bend.bent?.amount ?? App.fill("Action.particles.refract.amount"))
            onMoved: bend.put(pixels ? {
                "shift": Math.round(value)
            } : {
                "amount": Math.round(value * 100) / 100
            }, true)
        }
    }

    QQC2.ComboBox {
        readonly property var colors: ["none", "multiply", "add", "light"]

        visible: bend.bent !== null
        Layout.fillWidth: true
        model: [App.tr("Only bend what is behind"), App.tr("Multiply what is behind by their color"), App.tr("Add their color over what is behind"), App.tr("Add the bent picture, tinted by their color, as light over what is behind")]
        currentIndex: Math.max(0, colors.indexOf(bend.bent?.color ?? App.fill("Action.particles.refract.color")))
        onActivated: index => bend.put({
                "color": colors[index]
            })
    }

    Image {
        visible: bend.bent !== null && !!bend.info?.field
        source: bend.info?.field ?? ""
        sourceSize.width: 48
        sourceSize.height: 48
    }

    QQC2.CheckBox {
        visible: bend.bent !== null
        text: App.tr("Show the displacement over the preview")
        checked: SceneSwarmGlow.shown
        onToggled: SceneSwarmGlow.view(bend.page, checked)
    }

    SceneRepeater {
        id: listed

        rows: bend.bent !== null ? bend.info.levels : []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            font.bold: modelData.quality === SceneLook.quality
            text: modelData.refract === null ? App.tr("%1: left out", modelData.quality.toUpperCase()) : modelData.refract === 1 ? App.tr("%1: shifts up to %2 px at full size", modelData.quality.toUpperCase(), Math.round(bend.info.shift)) : App.tr("%1: shifts up to %2 px at half size", modelData.quality.toUpperCase(), Math.round(bend.info.shift))
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: bend.bent !== null
        wrapMode: Text.Wrap
        font.pixelSize: Look.tFoot
        text: bend.bent?.color === "none" ? App.tr("These particles are drawn only as the bend of what is behind them; Q1 and Q0, where phones often run, leave them out. Pick a color mode or draw the system with a second layer to paint them too.") : App.tr("The bent background is tinted by the color of each particle, its texture and its opacity; Q1 and Q0, where phones often run, leave these particles out.")
    }
}
