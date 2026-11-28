import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

FocusScope {
    id: code

    // Empty, the panel itself takes the keyboard (F6, Tab) and reads its hint; with a file, its controls do.
    readonly property bool keyTarget: file === null
    required property var page
    property var files: []
    property int at: -1
    property var drafts: ({})
    readonly property var file: files[at] ?? null
    readonly property var body: file?.kind === "shader" ? shader : script
    readonly property bool changed: file !== null && !!body && body.changed
    property bool scripting: false
    property bool shading: false
    readonly property var script: scriptLoad.item
    readonly property var shader: shaderLoad.item

    function key(entry) {
        return entry.kind + ":" + entry.name;
    }

    function keep() {
        if (file === null)
            return;
        const next = Object.assign({}, drafts);
        if (body.changed)
            next[key(file)] = body.area.text;
        else
            delete next[key(file)];
        drafts = next;
    }

    function show(index) {
        keep();
        at = index;
        scripting = scripting || file.kind === "script";
        shading = shading || file.kind === "shader";
        const draft = drafts[key(file)];
        if (file.kind === "shader")
            shader.show(file.name, draft);
        else
            script.show(file.name, draft);
        page.docks?.layout?.reveal("code");
    }

    function open(kind, name) {
        const found = files.findIndex(entry => entry.kind === kind && entry.name === name);
        if (found >= 0)
            return show(found);
        files = files.concat([{
                    "kind": kind,
                    "name": name
                }]);
        show(files.length - 1);
    }

    function openScript(path) {
        open("script", path);
    }

    function openShader(effect) {
        open("shader", effect);
    }

    function cycle(step) {
        if (files.length > 1)
            show((at + step + files.length) % files.length);
    }

    function save() {
        if (file === null)
            return;
        if (file.kind === "shader")
            shader.save(false);
        else
            script.save();
        keep();
    }

    function insert(text) {
        if (file?.kind === "script")
            script.insert(text);
    }

    activeFocusOnTab: file === null
    Accessible.role: Accessible.Pane
    Accessible.name: file === null ? App.tr("Open a script from Scripts, or the shader of an effect from Effects, to see its code here.") : ""
    // Ctrl+S anywhere in the panel (the file list, a button) saves the open file, not the scene.
    Keys.onShortcutOverride: event => event.accepted = code.file !== null && event.matches(StandardKey.Save)
    Keys.onPressed: event => {
        if (code.file !== null && event.matches(StandardKey.Save)) {
            code.save();
            event.accepted = true;
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Look.e2
        spacing: Look.e2

        RowLayout {
            Layout.fillWidth: true
            visible: code.file !== null

            QQC2.ComboBox {
                objectName: "codeFiles"
                Layout.fillWidth: true
                model: code.files.map(entry => entry.kind === "shader" ? App.tr("Shader %1", entry.name) : entry.name)
                currentIndex: code.at
                onActivated: index => code.show(index)
            }

            QQC2.Label {
                visible: code.changed
                text: App.tr("Not saved")
                color: Look.mist
            }

            QQC2.Button {
                flat: true
                icon.name: "document-save"
                text: App.tr("Save")
                enabled: code.changed
                onClicked: code.save()
            }

            QQC2.Button {
                flat: true
                icon.name: "document-open"
                text: App.tr("Open in your editor…")
                enabled: code.file?.kind === "script" || (code.shader?.path ?? "") !== ""
                onClicked: Qt.openUrlExternally("file://" + code.page.editor.folder + "/" + (code.file.kind === "shader" ? code.shader.path : code.file.name))
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: code.file === null
            text: App.tr("Open a script from Scripts, or the shader of an effect from Effects, to see its code here.")
            color: Look.mist
            wrapMode: Text.Wrap
            verticalAlignment: Text.AlignTop
        }

        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: code.file !== null
            currentIndex: code.file?.kind === "shader" ? 1 : 0

            Loader {
                id: scriptLoad

                active: code.scripting

                sourceComponent: SceneScriptBody {
                    page: code.page
                }
            }

            Loader {
                id: shaderLoad

                active: code.shading

                sourceComponent: SceneShaderBody {
                    page: code.page
                }
            }
        }
    }
}
