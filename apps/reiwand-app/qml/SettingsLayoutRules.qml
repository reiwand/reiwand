import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

SmallPanel {
    id: panel

    property var profile: null
    readonly property var auto: profile?.auto ?? {}
    readonly property var chosen: auto.outputs ?? []
    property var asked: null
    readonly property var shown: asked ?? chosen
    property string trouble: ""
    readonly property bool battery: App.state.battery_percent !== undefined && App.state.battery_percent !== null
    readonly property var names: Object.keys(App.state.outputs ?? {}).sort().concat(chosen.filter(name => !(name in (App.state.outputs ?? {}))))
    readonly property string power: auto.on_battery === true ? "battery" : auto.on_battery === false ? "plugged" : "any"

    function set(key, value) {
        trouble = "";
        profile && App.act("SetAuto", [profile.uid, key, value], "Profiles1", () => {}, failed => {
            asked = null;
            trouble = failed.message;
        });
    }

    function toggle(name) {
        asked = shown.includes(name) ? shown.filter(at => at !== name) : shown.concat([name]);
        set("outputs", asked.join(","));
    }

    onChosenChanged: asked = null

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
                    checked: panel.shown.includes(modelData)
                    onClicked: panel.toggle(modelData)
                }
            }
        }

        RowLayout {
            Kirigami.FormData.label: App.tr("Power") + ":"
            visible: panel.battery || panel.power !== "any"

            Repeater {
                model: [["any", App.tr("Any")], ["battery", App.tr("On battery")], ["plugged", App.tr("Plugged in")]]

                delegate: QQC2.Button {
                    required property var modelData

                    text: modelData[1]
                    checked: panel.power === modelData[0]
                    onClicked: panel.set("on_battery", modelData[0] === "any" ? "" : String(modelData[0] === "battery"))
                }
            }
        }

        QQC2.Label {
            Layout.preferredWidth: Kirigami.Units.gridUnit * 14
            visible: !panel.battery && panel.power !== "any"
            wrapMode: Text.Wrap
            text: App.tr("This computer has no battery, so this rule never holds. Choose “Any” to let the layout come by itself.")
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: panel.trouble !== ""
            type: Kirigami.MessageType.Error
            text: panel.trouble
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
