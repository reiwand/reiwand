import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: more

    required property var page
    required property var bloom
    readonly property var glare: bloom?.hdr ?? null
    readonly property var glareFields: [["threshold", App.tr("Glare threshold"), 0, 16, 1, 0.01], ["strength", App.tr("Glare strength"), 0, 8, 1, 0.01], ["knee", App.tr("Glare soft edge"), 0, 1, 0.5, 0.01], ["levels", App.tr("Glare levels"), 1, 8, 1, 1], ["scatter", App.tr("Glare scatter"), 0, 2, 0.5, 0.01]]

    function held(key, fallback) {
        const stored = glare === null ? undefined : glare[key];
        return typeof stored === "number" ? stored : fallback;
    }

    function plain(args, live) {
        page.run("canvas", {
                "bloom": args
            }, live);
    }

    function slid(key, value, step) {
        const rounded = step >= 1 ? Math.round(value) : Math.round(value * 100) / 100;
        if (rounded !== (glare[key] ?? -1))
            plain({
                    "hdr": {
                        [key]: rounded
                    }
                }, true);
    }

    SceneValueRow {
        text: App.tr("Radius in screen pixels")

        QQC2.Switch {
            Accessible.name: App.tr("Radius in screen pixels")
            checked: more.bloom !== null && more.bloom.radius_unit === "screen"
            onToggled: more.plain({
                    "radius_unit": checked ? "screen" : "canvas"
                })
        }
    }

    SceneValueRow {
        text: App.tr("Cap the brightest parts")

        QQC2.Switch {
            id: capped

            Accessible.name: App.tr("Cap the brightest parts")
            checked: typeof more.bloom?.limit === "number"
            onToggled: more.plain({
                    "limit": checked ? 16 : "none"
                })
        }
    }

    SceneValueRow {
        visible: capped.checked
        text: App.tr("Brightness cap")

        SceneSlider {
            Layout.fillWidth: true
            Layout.preferredWidth: Look.rielAncho
            Accessible.name: App.tr("Brightness cap")
            from: 0
            to: 16
            stepSize: 0.16
            value: more.bloom === null ? 16 : (typeof more.bloom.limit === "number" ? more.bloom.limit : 16)
            onMoved: {
                const rounded = Math.round(value * 100) / 100;
                if (rounded !== more.bloom.limit)
                    more.plain({
                            "limit": rounded
                        }, true);
            }
        }
    }

    SceneValueRow {
        text: App.tr("Glare above white")

        QQC2.Switch {
            Accessible.name: App.tr("Glare above white")
            checked: more.glare !== null
            onToggled: more.plain({
                    "hdr": checked ? {
                        "threshold": 1,
                        "strength": 1
                    } : "none"
                })
        }
    }

    Repeater {
        model: more.glareFields

        delegate: SceneValueRow {
            id: field

            required property var modelData

            visible: more.glare !== null
            text: field.modelData[1]

            SceneSlider {
                Layout.fillWidth: true
                Layout.preferredWidth: Look.rielAncho
                Accessible.name: field.text
                from: field.modelData[2]
                to: field.modelData[3]
                stepSize: field.modelData[5] >= 1 ? 1 : (field.modelData[3] - field.modelData[2]) / 100
                value: more.held(field.modelData[0], field.modelData[4])
                onMoved: more.slid(field.modelData[0], value, field.modelData[5])
            }
        }
    }
}
