import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

SmallPanel {
    id: question

    property string scene
    property string key
    property string name
    property var inputs: []
    property var kept: ({})
    readonly property var allowed: inputs.filter(input => question.kept[input] !== false)
    readonly property var left: inputs.filter(input => question.kept[input] === false)
    readonly property alias rows: rows

    signal answered(var allow, var deny)

    function ask(scene, key, name, inputs) {
        question.scene = scene;
        question.key = key;
        question.name = name;
        question.inputs = inputs;
        question.kept = {};
        open();
    }

    function answer(allow, deny) {
        App.act("Decide", [scene, key, allow, deny], "Inputs1", () => {}, error => String(error.name).endsWith(".Unconfirmed") ? applicationWindow().showPassiveNotification(App.tr("Confirm it in the desktop notification.")) : App.backend.lastError = error.message);
        answered(allow, deny);
    }

    title: App.tr("“%1” asks for permission to use", name)
    text: App.tr("It is already set and works without this. We remember it for next time.")
    quiet: App.tr("Without this")
    primary: App.tr("Allow")
    onAccepted: answer(allowed, left)
    onRejected: answer([], inputs)

    Repeater {
        id: rows

        model: question.inputs

        delegate: RowLayout {
            id: row

            required property string modelData
            readonly property alias pick: pick
            readonly property bool on: question.kept[modelData] !== false

            width: parent.width - 2 * Look.e4
            spacing: Look.e3

            Request {
                Layout.fillWidth: true
                icon: Permissions.icon(row.modelData)
                text: Permissions.title(row.modelData)
                reason: Permissions.reason(row.modelData)
            }

            QQC2.Switch {
                id: pick

                Layout.alignment: Qt.AlignTop
                checked: row.on
                Accessible.name: Permissions.title(row.modelData)
                onToggled: {
                    const next = Object.assign({}, question.kept);
                    next[row.modelData] = checked;
                    question.kept = next;
                }
            }
        }
    }
}
