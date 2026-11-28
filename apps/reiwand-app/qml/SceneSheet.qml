import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Wallpaper sheet: the package's own data (what the inspector shows for the package) in
// sections, with their names on the left to jump to each one.
Rectangle {
    id: sheet

    required property var page
    readonly property var rows: {
        sheet.page.view;
        sheet.page.fields;
        return JSON.parse(page.editor.inspect("/manifest") || "[]");
    }
    // A package that is not a scene puts its own settings (kind, entry page, framing, playback)
    // first.
    readonly property bool packaged: page.packagePage !== undefined
    readonly property var sections: (packaged ? [["package", App.tr("Package"), ""]] : []).concat([["data", App.tr("Data"), "identification-card"], ["rating", App.tr("Rating and flashes"), "lightning"], ["inputs", App.tr("Inputs and permissions"), "cursor-click"], ["device", App.tr("Device and platforms"), "monitor"], ["credits", App.tr("Credits and license"), "signature"], ["deps", App.tr("Dependencies"), "package"], ["advanced", App.tr("Advanced"), "gear-six"]])
    readonly property var places: ({
            "title": "data",
            "description": "data",
            "changes": "data",
            "version": "data",
            "authors": "data",
            "tags": "data",
            "accent": "data",
            "default_locale": "data",
            "content": "rating",
            "photosensitive": "rating",
            "inputs": "inputs",
            "permissions": "inputs",
            "web": "inputs",
            "requires": "device",
            "platforms": "device",
            "playback": "device",
            "display": "device",
            "license": "credits",
            "credits": "credits",
            "origin": "credits",
            "links": "credits"
        })
    property string current: packaged ? "package" : "data"

    function placeOf(row) {
        const parts = String(row.pointer).split("/");
        if (parts[2] === "requires" && parts[3] === "packages")
            return "deps";
        return places[parts[2]] ?? "advanced";
    }

    function rowsOf(key) {
        return rows.filter(row => placeOf(row) === key);
    }

    color: Look.night
    clip: true

    RowLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillHeight: true
            Layout.preferredWidth: Math.min(Look.sideLeft * 0.8, sheet.width * 0.3)
            color: DeskLook.stripTone

            // One Tab stop: the arrows move through the sections, Enter or Space goes to one.
            FocusList {
                id: nav

                anchors.fill: parent
                anchors.topMargin: Look.e2
                model: sheet.sections
                clip: true
                Keys.onReturnPressed: currentItem?.clicked()
                Keys.onEnterPressed: currentItem?.clicked()
                Keys.onSpacePressed: currentItem?.clicked()

                delegate: QQC2.ItemDelegate {
                    required property var modelData
                    required property int index

                    width: ListView.view.width
                    focusPolicy: Qt.NoFocus
                    implicitHeight: DeskLook.row
                    text: modelData[1]
                    highlighted: sheet.current === modelData[0]
                    onClicked: {
                        nav.currentIndex = index;
                        sheet.current = modelData[0];
                        const head = Array.from(column.children).find(item => item.section === modelData[0]);
                        if (head)
                            scroll.QQC2.ScrollBar.vertical.position = head.y / Math.max(1, column.height);
                    }
                }
            }
        }

        QQC2.ScrollView {
            id: scroll

            Layout.fillWidth: true
            Layout.fillHeight: true
            contentWidth: availableWidth

            ColumnLayout {
                id: column

                width: Math.min(scroll.availableWidth - Look.e4 * 2, Look.sideLeft * 3)
                x: Look.e4
                spacing: Look.e1

                Repeater {
                    model: sheet.sections

                    delegate: ColumnLayout {
                        required property var modelData
                        readonly property string section: modelData[0]

                        Layout.fillWidth: true
                        Layout.topMargin: Look.e3
                        visible: list.rows.length > 0 || modelData[0] === "package"
                        spacing: Look.e1

                        QQC2.Label {
                            text: modelData[1]
                            font.pixelSize: Look.tSection
                            font.weight: Font.Bold
                        }

                        QQC2.Label {
                            Layout.fillWidth: true
                            visible: modelData[0] === "data"
                            text: App.tr("What whoever finds the wallpaper sees in a repository and in their library. The texts are written in the base language; translations go in “Texts and languages”.")
                            color: Look.mist
                            font.pixelSize: Look.tFoot
                            wrapMode: Text.Wrap
                        }

                        Loader {
                            Layout.fillWidth: true
                            active: modelData[0] === "package"
                            visible: active

                            sourceComponent: PackageSections {
                                page: sheet.page
                            }
                        }

                        SceneFieldList {
                            id: list

                            Layout.fillWidth: true
                            page: sheet.page
                            rows: sheet.rowsOf(modelData[0])
                        }
                    }
                }
            }
        }
    }
}
