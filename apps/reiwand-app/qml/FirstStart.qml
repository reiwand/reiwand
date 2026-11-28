import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

FocusScope {
    id: start

    component Path: Column {
        id: path

        property string title
        property string text
        property bool main

        signal chosen

        spacing: Look.e2
        activeFocusOnTab: true
        Accessible.role: Accessible.Button
        Accessible.name: title
        Accessible.description: text
        Accessible.onPressAction: chosen()
        Keys.onReturnPressed: chosen()
        Keys.onEnterPressed: chosen()

        Text {
            text: path.title
            font.family: Look.titleFont
            font.pixelSize: path.main ? Look.tLarge : Look.tLarge - 2
            font.weight: path.main ? Font.Bold : Font.Normal
            font.letterSpacing: -0.02 * Look.tLarge
            font.underline: path.activeFocus || hover.hovered
            color: Look.frost
        }

        Text {
            width: path.main ? Look.coverFocus : Look.coverFocus * 0.9
            text: path.text
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            color: Look.mist
        }

        HoverHandler {
            id: hover

            cursorShape: Qt.PointingHandCursor
        }

        TapHandler {
            onTapped: path.chosen()
        }
    }

    required property var run
    required property var adder
    property var importer: null
    required property var page
    readonly property bool banded: (page.band?.width ?? 0) > 0
    // the other paths sit beside Explore only when they fit; otherwise they go under it
    readonly property bool besides: banded && explore.width + Look.e6 + side.implicitWidth <= width
    readonly property real low: Math.max(explore.height, side.y + side.height) + Look.e4 + (shelf.visible ? shelf.implicitHeight : 0)
    readonly property alias shelf: shelf
    readonly property var examples: run.shelf.slice(0, 12).map(pkg => ({
                "picture": run.previewed[run.spec(pkg)] ? App.backend.url(run.previewed[run.spec(pkg)]) : "",
                "name": pkg.title,
                "caption": pkg.kind === "scene" ? App.tr("Scene · Bundled examples") : App.tr("Bundled examples")
            }))
    // The bar follows the focus: the shelf of examples moves and sets, a path opens its page.
    readonly property var paths: [explore].concat(Array.from(side.children))
    readonly property bool onPath: paths.includes(Window.activeFocusItem)
    readonly property var hints: (onPath ? [{
                "keys": [App.trc("key", "Enter")],
                "text": App.tr("Open")
            }] : examples.length > 0 ? [{
                "keys": ["left", "right"],
                "text": App.tr("Move around")
            }, {
                "keys": [App.trc("key", "Enter")],
                "text": App.tr("Set as wallpaper")
            }] : []).concat([{
            "keys": [],
            "text": App.tr("You can also drop your files here")
        }])

    function put() {
        if (shelf.currentIndex >= 0 && shelf.currentIndex < run.shelf.length)
            run.place(run.shelf[shelf.currentIndex], false);
    }

    Item {
        id: lower

        y: start.banded ? Math.max(0, start.height - height) : 0
        width: start.width
        implicitHeight: shelf.y + (shelf.visible ? shelf.implicitHeight : 0)
        height: implicitHeight

        Path {
            id: explore

            main: true
            focus: !shelf.visible
            title: App.tr("Explore repositories")
            text: App.tr("Ready-made scenes to set, starting with the bundled examples.")
            onChosen: applicationWindow().reach(4)
        }

        Row {
            id: side

            x: start.besides ? parent.width - width : 0
            y: start.besides ? Math.max(0, explore.height - height) : explore.height + Look.e5
            spacing: Look.e6

            Path {
                title: App.tr("Add a source")
                text: App.tr("Pictures that renew themselves, like a picture of the day.")
                onChosen: applicationWindow().showPage(3)
            }

            Path {
                visible: !!start.importer
                title: App.tr("Import wallpaper…")
                text: App.tr("Wallpaper packages you downloaded, as .wpak files.")
                onChosen: start.importer?.pick()
            }

            Path {
                title: App.tr("Add a folder")
                text: App.tr("The pictures and videos you already have on this computer.")
                onChosen: start.adder.open()
            }
        }

        CoverShelf {
            id: shelf

            visible: start.examples.length > 0
            y: Math.max(explore.height, side.y + side.height) + Look.e4
            width: parent.width
            height: implicitHeight
            focus: true
            label: App.tr("Bundled examples")
            model: start.examples
            actions: Row {
                spacing: Look.e3

                QQC2.Button {
                    highlighted: true
                    icon.name: "image"
                    text: App.tr("Set as wallpaper")
                    onClicked: start.put()
                }

                PiecesKey {
                    anchors.verticalCenter: parent.verticalCenter
                    text: App.trc("key", "Enter")
                }
            }
            onActivated: start.put()
        }
    }
}
