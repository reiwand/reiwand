import QtQuick
import io.github.reiwand.app

Column {
    id: place

    required property string address
    property bool lost: false
    readonly property alias uploads: uploads
    readonly property alias keys: keys
    readonly property alias again: again

    spacing: Look.e3

    BrowseTerm {
        text: App.tr("Repository · %1", place.address.replace(/^[a-z]+:\/\//i, "").replace(/\/+$/, ""))
    }

    SignInAgain {
        id: again

        visible: place.lost
        repo: place.address
    }

    AccountUploads {
        id: uploads

        width: parent.width
        repo: place.address
        onSignedOut: place.lost = true
    }

    AccountKeys {
        id: keys

        width: parent.width
        repo: place.address
        onSignedOut: place.lost = true
    }
}
