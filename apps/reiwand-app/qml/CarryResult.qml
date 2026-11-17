import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: result

    property var added: ({})
    property var pending: []
    property int unresolved: 0
    property bool signIn: false
    readonly property var sections: Object.keys(added)

    signal leads(int page)

    function nothing() {
        return sections.length === 0 && pending.length === 0;
    }

    spacing: Look.e3

    Text {
        width: parent.width
        text: result.nothing() ? App.tr("Nothing was added: you already had all of it.") : App.tr("Added to your configuration:")
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    Repeater {
        model: result.sections

        Text {
            required property string modelData

            width: result.width
            text: App.tr("%1: %2", CarryWords.section(modelData), result.added[modelData])
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }
    }

    Text {
        visible: result.unresolved > 0
        width: parent.width
        text: App.tr("%1 items refer to wallpapers that are not on this computer.", result.unresolved)
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }

    Text {
        visible: result.pending.length > 0 || result.signIn
        width: parent.width
        text: App.tr("Still to do:")
        font.family: Look.letraTitulo
        font.pixelSize: Look.tCuerpo
        font.weight: Font.Bold
        color: Look.escarcha
    }

    Repeater {
        model: result.pending

        RowLayout {
            id: entry

            required property var modelData
            readonly property var lead: CarryWords.pending(modelData)

            width: result.width
            spacing: Look.e2

            Text {
                Layout.fillWidth: true
                text: modelData.name + ": " + modelData.reason
                wrapMode: Text.Wrap
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                color: Look.bruma
            }

            QQC2.Button {
                visible: entry.lead !== null
                text: entry.lead ? entry.lead.label : ""
                onClicked: result.leads(entry.lead.page)
            }
        }
    }

    RowLayout {
        visible: result.signIn
        width: parent.width
        spacing: Look.e2

        Text {
            Layout.fillWidth: true
            text: App.tr("Account sessions do not travel in the file.")
            wrapMode: Text.Wrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }

        QQC2.Button {
            text: App.tr("Sign in again")
            onClicked: result.leads(6)
        }
    }
}
