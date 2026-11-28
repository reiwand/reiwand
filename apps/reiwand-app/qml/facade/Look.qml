pragma Singleton
import QtQuick
import io.github.reiwand.app

QtObject {
    id: look

    property color night: "#0a1322"
    property color nightHigh: "#121e33"
    property color nightLine: "#2a3a57"
    property color mist: "#b9c4d6"
    property color mistLow: "#8f9db4"
    property color moon: "#9cc3f5"
    property color moonDeep: "#6f9fdc"
    property real veil: 0.8
    property bool light: false // the editor's light theme: ink text, the deep tones on paper
    readonly property color frost: light ? "#1c1a17" : "#f4efe6"
    readonly property color amber: light ? "#7a5200" : "#f0c26b"
    readonly property color coral: light ? "#a8361f" : "#ff9d8a"
    readonly property color sprout: light ? "#1d7040" : "#8fd6a8"
    readonly property color amberDeep: light ? "#f0c26b" : "#7a5200"
    readonly property color coralDeep: light ? "#ff9d8a" : "#a8361f"
    readonly property color sproutDeep: light ? "#8fd6a8" : "#1d7040"
    readonly property color veilColor: Qt.rgba(night.r, night.g, night.b, veil)

    readonly property string titleFont: "Red Hat Display"
    readonly property string textFont: "Onest"
    property string tier: "normal"
    // The editor (scene or package page, turned on by its status bar while in view) is denser: rows
    // of 26 to 30 px, 13 px type and nothing in a panel above its tab; the window's title is largest.
    property bool desk: false
    readonly property bool compact: tier === "compact" || tier === "short"
    readonly property bool roomy: tier === "roomy"

    readonly property real tCover: desk ? 13 : tier === "short" ? 40 : compact ? 52 : roomy ? 80 : 66
    readonly property real tPage: desk ? 13 : compact ? 42 : roomy ? 58 : 50
    readonly property real tLarge: desk ? 13 : compact ? 26 : roomy ? 38 : 34
    readonly property real tMenu: desk ? 13 : compact ? 22 : roomy ? 30 : 27
    readonly property real tFocus: desk ? 13 : compact ? 23 : roomy ? 30 : 27
    readonly property real tRow: desk ? 13 : compact ? 20 : roomy ? 24 : 22
    readonly property real tSection: desk ? 13 : 20
    readonly property real tAside: desk ? 12 : compact ? 16 : 18
    readonly property real tFilter: desk ? 13 : 18
    readonly property real tExplain: desk ? 12 : 17
    readonly property real tButton: desk ? 13 : 16.5
    readonly property real tName: desk ? 13 : 16
    readonly property real tBody: desk ? 13 : 15
    readonly property real tFoot: desk ? 12 : 13.5
    readonly property real tKey: desk ? 12 : 12.5

    readonly property int e1: 4
    readonly property int e2: 8
    readonly property int e3: 12
    readonly property int e4: 16
    readonly property int e5: 24
    readonly property int e6: 32
    readonly property int e7: 48
    readonly property int e8: 64
    readonly property int margin: compact ? 44 : roomy ? 72 : 56
    readonly property int marginTop: tier === "short" ? 18 : compact ? 30 : roomy ? 56 : 44
    readonly property int marginBottom: compact ? 24 : roomy ? 44 : 36

    readonly property int radius: 6
    readonly property int radiusSmall: 4
    readonly property int line: 1
    readonly property int ring: 2
    readonly property int ringGap: 3
    readonly property int ringRoom: ringGap + ring

    readonly property int menuWidth: compact ? 272 : roomy ? 360 : 312
    readonly property int menuHeight: compact ? 34 : roomy ? 52 : 46
    readonly property int asideHeight: compact ? 28 : roomy ? 42 : 38
    readonly property int cover: tier === "short" ? 160 : compact ? 200 : roomy ? 300 : 256
    readonly property int coverFocus: tier === "short" ? 240 : compact ? 300 : roomy ? 500 : 420
    readonly property int coverGrid: compact ? 200 : roomy ? 260 : 232
    readonly property int coverCreate: Math.round(cover * 0.62)
    readonly property int coverCreateFocus: Math.round(coverCreate * 1.35)
    readonly property int shelfGap: compact ? 22 : roomy ? 32 : 28
    readonly property int controlHeight: desk ? 28 : 40
    readonly property int buttonHeight: desk ? 30 : compact ? 38 : 42
    readonly property int rowHeight: compact ? 60 : roomy ? 80 : 74
    readonly property int sourceWho: compact ? 340 : 420
    readonly property int sourceGap: 14
    readonly property int sourcePhotos: 5
    readonly property real sourceVeil: 0.42
    readonly property int kindIcon: 28
    readonly property int sourceIcon: 26
    readonly property int kindsWidth: 1020
    readonly property int fieldHeight: desk ? 26 : 34
    readonly property int rowsWidth: compact ? 600 : roomy ? 740 : 660
    readonly property int explainWidth: compact ? 260 : roomy ? 340 : 300
    readonly property int shortcutsHeight: 26
    readonly property int veilBand: 190
    readonly property int keyWidth: 28
    readonly property int keyHeight: 26
    readonly property int keyRadius: 5
    readonly property int keyIcon: 13
    readonly property int logoLetter: 17
    readonly property int logoMoon: 14
    readonly property int markWidth: 16
    readonly property int markHeight: 4
    readonly property int menuIndent: 30
    readonly property int toastWidth: compact ? 440 : 480
    readonly property int topHeight: 52
    readonly property int tTop: 18
    readonly property int sideLeft: 280
    readonly property int canvasBarHeight: desk ? 34 : 44
    readonly property int sideRight: 320
    readonly property int canvasFootHeight: 168
    readonly property int topButtonHeight: 36
    readonly property int betweenHeight: 22
    readonly property int switchWidth: desk ? 30 : 40
    readonly property int switchHeight: desk ? 16 : 22
    readonly property int switchDot: desk ? 12 : 16
    readonly property int toolHeight: desk ? 24 : 32
    readonly property int optionHeight: desk ? 30 : 38
    readonly property int panelRadius: 10
    readonly property int railHeight: 4
    readonly property int railDot: desk ? 12 : 14
    readonly property int mark: 18
    readonly property int icon: desk ? 16 : 18
    readonly property int bar: 6
    readonly property real tTool: desk ? 12.5 : 14

    readonly property real tValue: desk ? 13 : 16.5
    readonly property string fixedFont: "monospace"
    readonly property int rowSide: 24
    readonly property int rowEnd: 20
    readonly property int explainGap: 52
    readonly property int valueGap: 10
    readonly property int statusIcon: 17
    readonly property int noteIcon: 15
    readonly property int toastIcon: 20
    readonly property int errorIcon: 28
    readonly property int errorIconLarge: 40
    readonly property int sublistHeight: 48
    readonly property int thumbnail: 56
    readonly property int closeHeight: 30
    readonly property int smallPanel: 360
    readonly property int screenTile: 112
    readonly property int screenTileWide: 164
    readonly property int screenMark: 4
    readonly property int menuSmall: 280
    readonly property int listGap: 6
    readonly property int fieldWidth: 260
    readonly property int emptyWidth: 760
    readonly property int errorWidth: 640
    readonly property int errorLargeWidth: 780
    readonly property int installProgress: 360
    readonly property int moreCard: 460
    readonly property int railWidth: 120
    readonly property int statusGap: 7
    readonly property int buttonGap: 9
    readonly property int pace: 180
    readonly property var paceCurve: [0.22, 1, 0.36, 1, 1, 1]

    readonly property real tCount: desk ? 12 : 13
    readonly property int filterSide: 12
    readonly property int filterGap: 7
    readonly property int sectionsSide: 14
    readonly property int markBottom: 2
    readonly property int searchWidth: 300
    readonly property int searchSide: 14
    readonly property int searchGap: 10
    readonly property int inUseHeight: 26
    readonly property int inUseGap: 6
    readonly property int coverMark: 10
    readonly property int chooseSide: 26
    readonly property int inset: 5
    readonly property int noteTop: 3
    readonly property int footTop: 5
    readonly property int actionsTop: 14
    readonly property real coverVeil: 0.42
    readonly property real coverVeilHover: 0.2
    readonly property real coverVeilChosen: 0.15
    readonly property real chooseVeil: 0.6
    readonly property real coverDimmed: 0.6
    readonly property real tight: -0.02

    readonly property string tones: [night, nightHigh, nightLine, frost, mist, mistLow, moon].map(String).join(",")
    readonly property Looks bridge: Looks {}

    function tone(set) {
        for (const name of ["night", "nightHigh", "nightLine", "mist", "mistLow", "moon", "moonDeep", "veil"]) {
            if (set[name] !== undefined)
                look[name] = set[name];
        }
    }

    function step(width, height) {
        return height <= 680 ? "short" : height <= 800 ? "compact" : width >= 1800 && height >= 1000 ? "roomy" : "normal";
    }

    onTonesChanged: bridge.paint(tones)
    onDeskChanged: bridge.dress(textFont, tBody)
    Component.onCompleted: {
        bridge.dress(textFont, tBody);
        bridge.paint(tones);
    }
}
