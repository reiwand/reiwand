import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

FocusScope {
    id: card

    required property var provider
    property bool rotating: false
    readonly property var kind: SourceKinds.of(provider.kind)
    readonly property string problem: provider.state === "needs_secret" ? App.tr("%1 rejected the key", kind.name) : provider.state === "locked" ? App.tr("The keyring is locked") : provider.state === "error" && !provider.offline ? said(provider.last_error) : ""
    readonly property string tone: provider.state === "locked" || provider.state === "needs_secret" ? "coral" : "amber"
    readonly property bool broken: problem !== ""
    readonly property bool bare: provider.items === 0
    readonly property bool syncing: provider.state === "syncing"

    signal rotate
    signal refresh
    signal details
    signal fix
    signal up
    signal down

    function said(error) {
        const code = /status (?:client|server) error \((\d+)/.exec(error || "");
        if (code)
            return App.tr("%1 answered with error %2", kind.name, code[1]);
        // the first line only: the rest (a plugin's stack, for one) is shown whole under Details
        return (error || "").split("\n")[0] || App.tr("It could not reach %1", kind.name);
    }

    function enter() {
        (primary.visible ? primary : refreshing).forceActiveFocus(Qt.TabFocusReason);
    }

    implicitHeight: Math.max(left.implicitHeight, Look.buttonHeight * 3)
    onActiveFocusChanged: activeFocus && Window.activeFocusItem === card && Qt.callLater(() => card && card.activeFocus && card.enter())
    Keys.onUpPressed: up()
    Keys.onDownPressed: down()

    Column {
        id: left

        width: Math.max(Look.sourceWho, acts.implicitWidth)
        spacing: Look.e2

        Row {
            spacing: Look.e2

            IconImage {
                anchors.verticalCenter: parent.verticalCenter
                name: card.kind.icon
                sourceSize: Qt.size(Look.sourceIcon, Look.sourceIcon)
                color: Look.moon
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: SourceKinds.heading(card.provider)
                font.family: Look.titleFont
                font.pixelSize: Look.tRow
                font.weight: Font.Bold
                color: Look.frost
            }
        }

        StatusLine {
            visible: card.broken
            text: card.problem
            tone: card.tone
        }

        Text {
            visible: card.broken && !card.bare
            text: App.tr("The %1 photos it already brought stay in your library.", card.provider.items)
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            color: Look.mist
        }

        Text {
            visible: !card.broken
            text: card.syncing ? App.tr("Bringing photos…") : card.bare && card.provider.offline ? App.tr("No connection: it has not brought photos yet") : card.bare && card.provider.last_sync ? App.tr("The last update found no photos") : card.bare ? App.tr("No photos yet") : App.tr("%1 photos, updated %2", card.provider.items, SourceKinds.ago(card.provider.last_sync))
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            color: Look.mist
        }

        Text {
            width: left.width
            text: App.tr("The photos belong to their authors. Use them as a wallpaper; do not publish them.")
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tFoot
            color: Look.mistLow
        }

        Row {
            id: acts

            spacing: Look.buttonGap

            QQC2.Button {
                id: primary

                visible: card.broken || (!card.bare && !card.rotating)
                highlighted: true
                text: card.broken ? (card.provider.state === "needs_secret" ? App.tr("Change key") : App.tr("Try again")) : App.tr("Rotate these photos")
                icon.name: card.broken ? (card.provider.state === "needs_secret" ? "key" : "arrows-clockwise") : "shuffle"
                onClicked: card.broken ? card.fix() : card.rotate()
            }

            Text {
                visible: card.rotating && !card.broken
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("They rotate on every screen")
                font.family: Look.textFont
                font.pixelSize: Look.tBody
                color: Look.sprout
            }

            QQC2.Button {
                id: refreshing

                flat: true
                enabled: !card.syncing && card.provider.state !== "needs_secret"
                text: App.tr("Update")
                icon.name: "arrows-clockwise"
                onClicked: card.refresh()
            }

            QQC2.Button {
                flat: true
                text: App.tr("Details")
                icon.name: "caret-right"
                onClicked: card.details()
            }
        }
    }

    SourceThumbs {
        anchors.left: left.right
        anchors.leftMargin: Look.sourceGap
        anchors.verticalCenter: left.verticalCenter
        height: Math.min(left.height, (card.width - left.width - Look.sourceGap - (Look.sourcePhotos - 1) * Look.e2) / Look.sourcePhotos * 9 / 16)
        source: card.provider.source
        stale: card.broken
        visible: !card.bare
    }
}
