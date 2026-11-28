import QtQuick
import io.github.reiwand.app

SmallPanel {
    id: panel

    property var profile: null

    title: App.tr("New name for the layout")
    quiet: App.tr("Cancel")
    primary: App.tr("Rename")
    primaryIcon: "pencil-simple"
    onAccepted: panel.profile && name.fieldText !== "" && App.act("Rename", [panel.profile.uid, name.fieldText], "Profiles1")
    onProfileChanged: name.fieldText = profile ? profile.name : ""

    ListRow {
        id: name

        width: parent ? parent.width : implicitWidth
        text: App.tr("Name")
        kind: "field"
        placeholder: App.tr("No name")
    }
}
