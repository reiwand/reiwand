pragma Singleton
import QtQuick
import io.github.reiwand.app

// Measures of the editor workbench frame: its top bar, tab strips and status bar, denser than
// the rest of the app (rows of 28 to 30 px and 13.5 px type).
QtObject {
    readonly property color stripTone: Look.light ? Qt.darker(Look.night, 1.06) : Qt.darker(Look.night, 1.8)
    // Around the canvas the editor stays neutral, whatever the tones of the scene.
    readonly property color ground: Look.light ? "#e4e4e2" : "#0f1012"
    readonly property int top: 44
    readonly property int status: 28
    readonly property int strip: 32
    readonly property int word: 28
    readonly property int button: 30
    readonly property int layout: 32
    readonly property int wordSide: 9
    readonly property int tabSide: 10
    readonly property int gap: 6
    readonly property real tWord: 13.5
    readonly property real tTitle: 15
    readonly property real tTab: 13
    readonly property real tLayout: 13
    readonly property real tStatus: 12.5
    readonly property real tCap: 11
    readonly property int tabIcon: 14
    readonly property int statusIcon: 14
    readonly property int planWidth: 18
    readonly property int planHeight: 12
    readonly property int planMenuWidth: 30
    readonly property int planMenuHeight: 20
    readonly property real planFill: 0.35
    readonly property int capHeight: 18
    readonly property int capWidth: 20
    readonly property int capSide: 5
    readonly property int markHeight: 3
    readonly property int markBottom: 3
    readonly property int sepHeight: 14
    readonly property int statusGap: 14
    readonly property int folds: 1500
    readonly property int menuRow: 30
    readonly property int row: 30
    readonly property int label: 112
    readonly property int rowButton: 24
    readonly property int rowRoomy: 230
    readonly property int thumb: 22
    readonly property int thumbWide: 32
    readonly property real tRow: 13
    readonly property int barRow: 34
    readonly property real tBar: 12.5
    readonly property real tKind: 12
    readonly property real tMenu: 13.5
}
