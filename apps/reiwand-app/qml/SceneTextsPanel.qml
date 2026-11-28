import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Texts and languages panel: the base language and every text of the package that is shown to
// people (title, description, notes of the version, the package's own strings), each with its
// translations.
Rectangle {
    id: texts

    required property var page
    readonly property var rows: {
        texts.page.view;
        texts.page.fields;
        return JSON.parse(page.editor.inspect("/manifest") || "[]").filter(row => ["default_locale", "title", "description", "changes", "strings"].includes(String(row.pointer).split("/")[2]));
    }

    color: Look.night
    clip: true

    QQC2.ScrollView {
        id: scroll

        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e2
        anchors.topMargin: Look.e2
        contentWidth: availableWidth

        ScrollColumn {
            view: scroll

            QQC2.Label {
                Layout.fillWidth: true
                text: App.tr("The texts are written in the base language; each one adds its translations here.")
                color: Look.mist
                font.pixelSize: Look.tFoot
                wrapMode: Text.Wrap
            }

            SceneFieldList {
                Layout.fillWidth: true
                page: texts.page
                rows: texts.rows
            }
        }
    }
}
