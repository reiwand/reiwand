import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: body

    required property var page
    property string path: ""
    property string saved: ""
    property var report: ({})
    readonly property alias area: area
    readonly property string stamp: JSON.stringify((page.view.scripts ?? []).filter(s => s.path === body.path))
    readonly property bool changed: area.text !== body.saved

    function audit() {
        body.report = JSON.parse(page.editor.code("audit", JSON.stringify({
                        "path": body.path
                    })) || "{}");
    }

    function load() {
        body.saved = JSON.parse(page.editor.code("source", JSON.stringify({
                        "path": body.path
                    })) || "\"\"");
        area.text = body.saved;
        body.audit();
    }

    function show(path, draft) {
        body.path = path;
        body.load();
        if (draft !== undefined)
            area.text = draft;
    }

    function save() {
        page.run("script", {
                "write": {
                    "path": body.path,
                    "text": area.text
                }
            });
        if (page.editor.error === "")
            body.load();
    }

    function insert(code) {
        area.insert(area.cursorPosition, code + "\n");
        area.forceActiveFocus();
    }

    function jump(line) {
        const lines = area.text.split("\n");
        let at = 0;
        for (let i = 0; i < Math.min(line - 1, lines.length); i++)
            at += lines[i].length + 1;
        area.cursorPosition = at;
        area.forceActiveFocus();
    }

    spacing: Look.e2
    onStampChanged: {
        if (body.path === "")
            return;
        if (body.changed)
            body.audit();
        else
            body.load();
    }

    SceneScriptAudit {
        Layout.fillWidth: true
        page: body.page
        path: body.path
        report: body.report
    }

    SceneScriptRigMissing {
        Layout.fillWidth: true
        report: body.report
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
        }
    }
}
