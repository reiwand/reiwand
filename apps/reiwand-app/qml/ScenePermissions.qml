import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

ColumnLayout {
    id: permits

    required property string uid
    property var inputs: ({})
    property string screen: App.outputs.find(name => ((App.state.outputs ?? {})[name] ?? {}).item === permits.uid) ?? ""
    readonly property var uses: (inputs.shown ?? {})[screen] ?? {}
    readonly property var granted: uses.granted ?? []
    readonly property var wanted: Permissions.asked.filter(input => permits.granted.includes(input) || (permits.uses.without ?? []).includes(input) || (permits.uses.undecided ?? []).includes(input))

    function load() {
        App.query("GetInputs", [], "Inputs1", reply => permits && (permits.inputs = reply));
    }

    function decide(input, allow) {
        App.act("Decide", [uses.scene, uses.key ?? "", allow ? [input] : [], allow ? [] : [input]], "Inputs1", () => permits && permits.load(), error => {
            if (!permits)
                return;
            if (String(error.name).endsWith(".Unconfirmed"))
                applicationWindow().showPassiveNotification(App.tr("Confirm it in the desktop notification."));
            permits.load();
        });
    }

    visible: wanted.length > 0
    spacing: 0
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "inputs" || topic === "state")
                permits.load();
        }
    }

    Section {
        text: App.tr("Permissions of this scene")
    }

    Repeater {
        model: permits.wanted

        delegate: ListRow {
            id: row

            required property string modelData

            Layout.fillWidth: true

            Connections {
                target: permits

                function onInputsChanged() {
                    row.checked = Qt.binding(() => permits.granted.includes(row.modelData));
                }
            }

            kind: "switch"
            text: Permissions.title(modelData)
            explain: Permissions.reason(modelData)
            checked: permits.granted.includes(modelData)
            value: checked ? App.tr("Allowed") : App.tr("Not allowed")
            onToggled: permits.decide(modelData, checked)
        }
    }
}
