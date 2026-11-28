import QtQuick
import io.github.reiwand.app

FocusScope {
    id: tab

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property int mature: options["library.hide_mature"] === true ? 0 : options["library.show_mature"] === true ? 2 : 1
    readonly property var lost: page.sources.filter(source => !!source.error)
    readonly property var plays: page.outputs.map(name => [name, ((App.state.outputs ?? {})[name] ?? {}).playlist ?? null])
    readonly property var locked: [page.lock.every ?? null].concat(Object.values(page.lock.outputs ?? {})).filter(own => own !== null)
    readonly property string playing: plays.length === 1 ? (plays[0][1] ? plays[0][1].name : App.trc("list", "None")) : plays.map(pair => App.tr("%1: %2", App.screen(pair[0]), pair[1] ? pair[1].name : App.tr("none"))).join(" · ")
    readonly property var rows: [
        {
            "text": adult.text,
            "value": adult.options[adult.current]
        },
        {
            "text": flashes.text,
            "value": flashes.options[flashes.current],
            "explain": flashes.explain
        },
        {
            "text": folders.text,
            "value": folders.value
        },
        {
            "text": lists.text,
            "value": lists.value
        },
        {
            "text": App.tr("List that plays on each screen"),
            "value": lists.value,
            "place": "lists"
        },
        {
            "text": lock.text,
            "value": lock.value
        },
        {
            "text": more.text,
            "value": more.note
        },
        {
            "text": light.text,
            "value": light.value,
            "explain": light.explain
        },
        {
            "text": App.tr("Add a folder…"),
            "place": "folders"
        },
        {
            "text": App.tr("Include subfolders"),
            "place": "folders"
        },
        {
            "text": App.tr("Change wallpaper with the keyboard"),
            "place": "more"
        },
        {
            "text": App.tr("Previous wallpaper with the keyboard"),
            "place": "more"
        },
        {
            "text": App.tr("Apply the wallpaper of the switcher"),
            "place": "more"
        },
        {
            "text": App.tr("Close the switcher without changing"),
            "place": "more"
        },
        {
            "text": App.tr("Keyboard shortcuts"),
            "place": "more"
        },
        {
            "text": App.tr("News from the repositories"),
            "place": "more"
        },
        {
            "text": App.tr("Thumbnail cache"),
            "place": "more"
        },
        {
            "text": App.tr("Show the number of votes"),
            "place": "more"
        },
        {
            "text": App.tr("Watch the library"),
            "place": "more"
        }
    ]

    implicitHeight: column.height

    ListRows {
        id: column

        Section {
            text: App.tr("What is shown")
        }

        ListRow {
            id: adult

            text: App.tr("Mature content")
            kind: "choice"
            options: [App.tr("Hidden"), App.tr("Blurred"), App.tr("Visible")]
            current: tab.mature
            explain: App.tr("Hidden leaves it out of every list. Blurred shows its cover blurred until you open it. Visible shows it as it is.")
            onChosen: index => {
                App.act("SetOption", ["library.hide_mature", String(index === 0)]);
                App.act("SetOption", ["library.show_mature", String(index === 2)]);
            }
        }

        ListRow {
            id: flashes

            text: App.tr("Flashes")
            kind: "choice"
            options: [App.tr("Disabled"), App.tr("Allowed")]
            current: tab.options["playback.allow_flashing"] === true ? 1 : 0
            explain: current === 0 ? App.tr("Wallpapers with flashes or sudden changes of light do not play, and the library marks them with “Has flashes, disabled”.") : App.tr("Wallpapers with flashes or sudden changes of light play like any other.")
            onChosen: index => App.act("SetOption", ["playback.allow_flashing", String(index === 1)])
        }

        Section {
            text: App.tr("Your library")
        }

        ListRow {
            id: folders

            text: App.tr("Library folders")
            kind: "page"
            value: tab.page.sources.length === 1 ? App.tr("1 folder") : App.tr("%1 folders", tab.page.sources.length)
            note: tab.lost.length > 0 ? App.tr("%1 cannot be found", tab.lost.map(source => SettingsFolder.name(source)).join(", ")) : ""
            noteTone: "coral"
            onOpened: tab.page.open("folders", folders)
        }

        ListRow {
            id: lists

            text: tab.page.outputs.length > 1 ? App.tr("Lists per screen") : App.tr("List that plays")
            kind: "page"
            value: tab.playing
            onOpened: tab.page.open("lists", lists)
        }

        ListRow {
            id: lock

            text: App.tr("Lock screen")
            kind: "page"
            value: tab.locked.length === 0 ? App.tr("Like the desktop") : tab.locked.map(own => own.name).join(" · ")
            onOpened: tab.page.open("lock", lock)
        }

        ListRow {
            id: more

            text: App.tr("More")
            note: App.tr("Keyboard shortcut, news, thumbnails, votes, watching folders")
            kind: "page"
            onOpened: tab.page.open("more", more)
        }

        Section {
            text: App.tr("Making wallpapers")
        }

        ListRow {
            id: light

            text: App.tr("Light editor")
            kind: "switch"
            checked: EditorLight.saved.light
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Dark ink on light paper, in the editor only. Its View menu changes it too.")
            onToggled: EditorLight.saved.light = checked
        }
    }
}
