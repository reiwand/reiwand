import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: body

    required property var page
    property string effect: ""
    property string saved: ""
    property string descriptor: ""
    property string path: ""
    property string stage: "layer"
    property var report: ({})
    readonly property alias area: area
    readonly property bool changed: area.text !== body.saved
    readonly property int line: report.line ?? 0
    readonly property string chosenLayer: page.editor.selected.startsWith("/scene/layers") ? (JSON.parse(page.editor.value(page.editor.selected + "/id") || "null") ?? "") : ""

    function load() {
        const source = JSON.parse(page.editor.rig("", JSON.stringify({
                        "shader_source": {
                            "effect": body.effect
                        }
                    })) || "null") ?? {};
        body.saved = source.wgsl ?? "";
        body.descriptor = source.descriptor ?? "";
        body.path = source.wgsl_path ?? "";
        body.stage = source.stage ?? "layer";
        area.text = body.saved;
        checker.check(body.effect, body.descriptor, area.text);
    }

    function show(effect, draft) {
        body.effect = effect;
        body.report = {};
        body.load();
        if (draft !== undefined)
            area.text = draft;
    }

    function save(gesture) {
        page.run("shader", {
                "write": {
                    "effect": body.effect,
                    "wgsl": area.text
                }
            }, gesture);
        if (page.editor.error === "")
            body.saved = area.text;
    }

    function start(line) {
        const lines = area.text.split("\n");
        let at = 0;
        for (let i = 0; i < Math.min(line - 1, lines.length); i++)
            at += lines[i].length + 1;
        return at;
    }

    function reported(text) {
        body.report = JSON.parse(text || "{}");
        if (body.report.ok && live.checked && body.changed)
            body.save(true);
    }

    function jump() {
        area.cursorPosition = body.start(body.line) + Math.max(0, (body.report.column ?? 1) - 1);
        area.forceActiveFocus();
    }

    spacing: Look.e2

    ShaderCheck {
        id: checker

        onReportChanged: body.reported(report)
    }

    Timer {
        id: settle

        interval: 500
        onTriggered: checker.check(body.effect, body.descriptor, area.text)
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: true
        type: checker.checking ? Kirigami.MessageType.Information : body.report.ok ? Kirigami.MessageType.Positive : Kirigami.MessageType.Error
        text: checker.checking ? App.tr("Checking…") : body.report.ok ? App.tr("Builds; costs %1 units", Math.round((body.report.units ?? 0) * 100) / 100) : body.line > 0 ? App.tr("Line %1, column %2: %3", body.line, body.report.column ?? 1, body.report.error ?? "") : (body.report.error ?? "")
        actions: [
            Kirigami.Action {
                text: App.tr("Go to the line")
                icon.name: "go-jump"
                visible: body.line > 0
                onTriggered: body.jump()
            }
        ]
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.CheckBox {
            id: live

            Layout.fillWidth: true
            text: App.tr("Preview while typing: write each version that builds")
        }

        QQC2.Button {
            flat: true
            icon.name: "list-add"
            text: App.tr("Use on the selected layer")
            visible: body.stage === "layer"
            enabled: body.chosenLayer !== ""
            onClicked: body.page.run("effects", {
                    "add": {
                        "layer": body.chosenLayer,
                        "effect": body.effect
                    }
                })
        }
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true

        Area {
            id: area

            wrapMode: TextEdit.NoWrap
            font.family: "monospace"
            selectByMouse: true
            // Ctrl+S saves this file (the Code panel's Save), not the scene, while the text has focus.
            Keys.onShortcutOverride: event => event.accepted = event.matches(StandardKey.Save)
            onTextChanged: settle.restart()

            Rectangle {
                readonly property rect at: area.positionToRectangle(body.start(body.line))

                visible: body.line > 0 && !body.report.ok
                x: 0
                y: at.y
                width: area.width
                height: at.height
                color: Look.coral
                opacity: 0.5
                z: -1
            }
        }
    }
}
