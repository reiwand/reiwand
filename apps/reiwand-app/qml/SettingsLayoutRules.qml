import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

PanelChico {
    id: panel

    property var profile: null
    readonly property var auto: profile?.auto ?? {}
    readonly property var chosen: auto.outputs ?? []
    readonly property var names: Object.keys(App.state.outputs ?? {}).sort().concat(chosen.filter(name => !(name in (App.state.outputs ?? {}))))
    readonly property string power: auto.on_battery === true ? "battery" : auto.on_battery === false ? "plugged" : "any"

    function set(key, value) {
        profile && App.act("SetAuto", [profile.uid, key, value], "Profiles1");
    }

    function toggle(name) {
        set("outputs", (chosen.includes(name) ? chosen.filter(at => at !== name) : chosen.concat([name])).join(","));
    }

    title: profile ? App.tr("When “%1” comes by itself", profile.name) : ""
    text: App.tr("The layout is used on its own when these are true. With nothing set, only when you choose it.")
    quiet: App.tr("Close")

    Kirigami.FormLayout {
        width: parent ? parent.width : implicitWidth

        Flow {
            Kirigami.FormData.label: App.tr("Screens") + ":"
            Layout.preferredWidth: Kirigami.Units.gridUnit * 14
            spacing: Kirigami.Units.smallSpacing

            Repeater {
                model: panel.names

                delegate: QQC2.Button {
                    required property string modelData

                    text: App.screen(modelData, panel.names)
                    checkable: true
                    checked: panel.chosen.includes(modelData)
                    onClicked: panel.toggle(modelData)
                }
            }
        }

        RowLayout {
            Kirigami.FormData.label: App.tr("Power") + ":"

            Repeater {
                model: [["any", App.tr("Any")], ["battery", App.tr("On battery")], ["plugged", App.tr("Plugged in")]]

                delegate: QQC2.Button {
                    required property var modelData

                    text: modelData[1]
                    checkable: true
                    autoExclusive: true
                    checked: panel.power === modelData[0]
                    onClicked: panel.set("on_battery", modelData[0] === "any" ? "" : String(modelData[0] === "battery"))
                }
            }
        }

        Settings {
            daemon: App
            model: (App.schema.auto ?? []).filter(field => field.key === "priority")
            values: ({
                    "priority": panel.profile?.priority ?? 0
                })
            onEdited: (key, value) => panel.set(key, value)
        }
    }
}
