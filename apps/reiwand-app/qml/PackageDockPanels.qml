import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The panels of a package's workbench, each a unit made once that goes to the group holding it:
// the preview, the files, its options or presets, Try, the inspector, the player, problems and the
// four panels of the wallpaper.
Item {
    id: panels

    required property var area
    readonly property var page: area.page

    component Unit: SceneUnit {
        area: panels.area
    }

    component Pane: Rectangle {
        anchors.fill: parent
        color: Look.night
        clip: true
    }

    Unit {
        unit: "canvas"

        PackagePreview {
            anchors.fill: parent
            page: panels.page
        }
    }

    Unit {
        unit: "assets"

        Pane {
            PackageFiles {
                anchors.fill: parent
                page: panels.page
            }
        }
    }

    Unit {
        unit: "options"

        Pane {
            Loader {
                anchors.fill: parent
                sourceComponent: panels.page.kind === "video" ? presets : options
            }

            Component {
                id: options

                SceneProperties {
                    page: panels.page
                }
            }

            Component {
                id: presets

                PackagePresets {
                    page: panels.page
                }
            }
        }
    }

    Unit {
        unit: "try"

        Pane {
            SceneTry {
                anchors.fill: parent
                page: panels.page
            }
        }
    }

    Unit {
        unit: "properties"

        Pane {
            PackageInspector {
                anchors.fill: parent
                anchors.margins: Look.e2
                page: panels.page
            }
        }
    }

    Unit {
        unit: "inputs"

        Pane {
            QQC2.ScrollView {
                id: player

                anchors.fill: parent
                contentWidth: availableWidth

                PackageMedia {
                    width: player.availableWidth
                    page: panels.page
                }
            }
        }
    }

    Unit {
        unit: "problems"

        Pane {
            SceneProblems {
                anchors.fill: parent
                anchors.margins: Look.e3
                page: panels.page
            }
        }
    }

    Unit {
        unit: "console"

        Pane {
            SceneConsolePanel {
                anchors.fill: parent
                page: panels.page
            }
        }
    }

    SceneDockSheets {
        area: panels.area
    }
}
