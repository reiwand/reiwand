import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

FocusScope {
    id: panel

    required property var page
    readonly property string output: page.editor.output
    property var lines: App.state.previews?.[panel.output]?.console ?? []
    property string scriptError: App.state.previews?.[panel.output]?.error ?? ""
    property string kept: "all"
    readonly property var shown: lines.filter(line => kept === "all" || (kept === "errors" ? /^error\b/.test(line) : /^(warn|error)\b/.test(line)))
    readonly property string errorFile: (scriptError.match(/([\w\/.-]+\.(?:js|mjs|ts)):\d+/) ?? [])[1] ?? ""
    readonly property int errorLine: Number((scriptError.match(/line (\d+)/) ?? scriptError.match(/:(\d+)(?::\d+)?\b/) ?? [])[1] ?? 0)

    function fileOf(line) {
        return (line.match(/^\w+ ([^:\s]+\.(?:js|mjs|ts)):/) ?? [])[1] ?? "";
    }

    function open(file, line) {
        const code = page.docks?.code;
        if (!code || file === "")
            return;
        code.openScript(file);
        if (line > 0)
            code.script.jump(line);
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Look.e2
        spacing: Look.e2

        RowLayout {
            Layout.fillWidth: true

            Repeater {
                model: [["all", App.tr("All")], ["warnings", App.tr("Warnings")], ["errors", App.tr("Errors")]]

                delegate: QQC2.ToolButton {
                    required property var modelData

                    objectName: "console-" + modelData[0]
                    text: modelData[1]
                    checkable: true
                    checked: panel.kept === modelData[0]
                    onClicked: panel.kept = modelData[0]
                }
            }

            Item {
                Layout.fillWidth: true
            }
        }

        Failure {
            objectName: "scriptError"
            Layout.fillWidth: true
            visible: panel.scriptError !== ""
            title: panel.errorLine > 0 ? App.tr("Error in the script, line %1", panel.errorLine) : App.tr("Error in the script")
            text: panel.scriptError
            action: panel.errorFile !== "" ? App.tr("Go to the line") : ""
            onActed: panel.open(panel.errorFile, panel.errorLine)
        }

        QQC2.Label {
            Layout.fillWidth: true
            Layout.fillHeight: true
            verticalAlignment: Text.AlignTop
            visible: panel.shown.length === 0 && panel.scriptError === ""
            text: panel.lines.length === 0 ? App.tr("The scripts of the preview have written nothing yet.") : App.tr("No line of this kind.")
            color: Look.mist
            wrapMode: Text.Wrap
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: panel.shown.length > 0

            ListView {
                id: list

                objectName: "consoleLines"
                model: panel.shown
                clip: true

                delegate: QQC2.ItemDelegate {
                    id: row

                    required property string modelData

                    width: ListView.view.width
                    text: modelData
                    onClicked: panel.open(panel.fileOf(modelData), 0)

                    contentItem: QQC2.Label {
                        text: row.modelData
                        textFormat: Text.PlainText
                        font.family: "monospace"
                        elide: Text.ElideRight
                        color: /^error\b/.test(row.modelData) ? Look.coral : /^warn\b/.test(row.modelData) ? Look.amber : Look.frost
                    }
                }
            }
        }

        Item {
            Layout.fillHeight: true
            visible: panel.shown.length === 0 && panel.scriptError !== ""
        }
    }
}
