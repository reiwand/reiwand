import QtQuick
import io.github.reiwand.app

FocusScope {
    id: tab

    required property var page
    property var lightsStatus: ({})
    readonly property var options: App.state.options ?? {}
    readonly property var inputs: page.inputs
    readonly property int granted: Object.keys(inputs.grants ?? {}).length
    readonly property bool lightsOn: options["rgb.enabled"] === true
    readonly property var rows: [
        {
            "text": cut.text,
            "value": cut.value
        },
        {
            "text": grants.text,
            "value": grants.value
        },
        {
            "text": sound.text,
            "value": sound.value
        },
        {
            "text": media.text,
            "value": media.value
        },
        {
            "text": lights.text,
            "value": lights.value,
            "off": lights.off
        },
        {
            "text": move.text,
            "value": move.value
        },
        {
            "text": advanced.text,
            "value": advanced.value
        }
    ].concat(SettingsMachineRows.places())

    function loadLights() {
        App.query("GetStatus", [], "Rgb1", reply => tab && (tab.lightsStatus = reply ?? {}));
    }

    implicitHeight: column.height
    onVisibleChanged: if (visible)
        loadLights()
    Component.onCompleted: loadLights()

    ListRows {
        id: column

        ListRow {
            id: cut

            text: App.tr("Private mode")
            kind: "switch"
            checked: App.cut
            value: checked ? App.tr("On") : App.tr("Off")
            explain: App.tr("Cuts all network: sources and repositories are not updated, covers are not downloaded, web wallpapers load only files of this computer and nothing talks to your server or your phone.")
            onToggled: App.act("SetOption", ["network.private", String(checked)], "Daemon1")
        }

        ListRow {
            id: grants

            text: App.tr("Scene permissions")
            kind: "page"
            value: MachineWords.scenes(tab.granted)
            explain: App.tr("What each scene can use of this computer: the pointer, the sound, the music that plays. You take it back from here whenever you want.")
            status: MachineWords.following(tab.inputs)
            onOpened: tab.page.open("grants", grants)
        }

        ListRow {
            id: sound

            text: App.tr("Sound of the scenes")
            kind: "page"
            value: MachineWords.sound(tab.options)
            onOpened: tab.page.open("sound", sound)
        }

        ListRow {
            id: media

            text: App.tr("What you are listening to")
            note: App.tr("Scenes can show the song that plays")
            kind: "page"
            value: tab.options["media.remote_covers"] === true ? App.tr("With covers from the internet") : App.tr("Without covers from the internet")
            onOpened: tab.page.open("media", media)
        }

        ListRow {
            id: lights

            text: App.tr("RGB lights")
            note: off ? App.tr("They are handled by OpenRGB, which is not running on this computer") : App.tr("Keyboards, mice and other lights follow the wallpaper")
            kind: off ? "text" : "page"
            off: !MachineWords.lightsFound(tab.lightsOn, tab.lightsStatus)
            value: MachineWords.lights(tab.lightsOn, tab.lightsStatus)
            explain: off ? App.tr("Start OpenRGB and its server, or search for “OpenRGB” to change its address or turn the lights off.") : ""
            onOpened: tab.page.open("lights", lights)
        }

        ListRow {
            id: move

            text: App.tr("Move your configuration")
            kind: "page"
            value: App.tr("Export and import")
            explain: App.tr("Take your repositories, lists, favorites and settings to another computer in one file, or bring them here from one.")
            onOpened: tab.page.open("carry", move)
        }

        ListRow {
            id: advanced

            text: App.tr("Advanced")
            kind: "page"
            value: App.tr("Graphics card, HDR, version")
            onOpened: tab.page.open("advanced", advanced)
        }
    }
}
