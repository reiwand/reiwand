import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

SmallPanel {
    id: report

    property var failed: []
    property var newer: []
    property var added: []
    readonly property var rows: failed.map(one => ({
                    "path": one.path,
                    "why": one.error
                })).concat(newer.map(one => ({
                        "path": one.path,
                        "why": App.tr("Your library has the newer version %1.", one.version)
                    })))

    signal older(var paths)
    signal see(var uids)

    function tell(output) {
        failed = output.failed ?? [];
        newer = (output.kept ?? []).filter(one => one.newer);
        added = (output.imported ?? []).map(one => one.uid);
        open();
    }

    title: failed.length === 0 ? App.tr("Already in your library") : failed.length === 1 ? App.tr("1 file was not added") : App.tr("%1 files were not added", failed.length)
    text: newer.length === 0 ? "" : newer.length === 1 ? App.tr("1 file is an older version of a wallpaper you have.") : App.tr("%1 files are older versions of wallpapers you have.", newer.length)
    quiet: App.tr("Close")
    primary: newer.length === 0 ? "" : newer.length === 1 ? App.tr("Use the older one") : App.tr("Use the older ones")
    first: list
    onAccepted: older(newer.map(one => one.path))
    foot: [
        QQC2.Button {
            flat: true
            visible: report.added.length > 0
            text: report.added.length === 1 ? App.tr("See the added one") : App.tr("See the %1 added", report.added.length)
            onClicked: {
                report.close();
                report.see(report.added);
            }
        }
    ]

    ListView {
        id: list

        width: Look.smallPanel - 2 * Look.e4
        height: Math.min(contentHeight, Look.e6 * 8)
        clip: true
        spacing: Look.e2
        model: report.rows
        keyNavigationEnabled: true
        highlightFollowsCurrentItem: true
        boundsBehavior: Flickable.StopAtBounds
        Accessible.role: Accessible.List
        Accessible.name: report.title
        QQC2.ScrollBar.vertical: QQC2.ScrollBar {
        }

        delegate: Column {
            id: row

            required property var modelData
            required property int index

            width: ListView.view.width
            Accessible.role: Accessible.ListItem
            Accessible.name: name.text + ": " + why.text

            Text {
                id: name

                width: parent.width
                text: row.modelData.path.split("/").pop()
                elide: Text.ElideMiddle
                font.family: Look.textFont
                font.pixelSize: Look.tBody
                font.weight: row.ListView.isCurrentItem && list.activeFocus ? Font.Bold : Font.DemiBold
                color: Look.frost
            }

            Text {
                id: why

                width: parent.width
                text: row.modelData.why
                wrapMode: Text.Wrap
                maximumLineCount: 4
                elide: Text.ElideRight
                font.family: Look.textFont
                font.pixelSize: Look.tBody
                color: Look.mist
            }
        }
    }
}
