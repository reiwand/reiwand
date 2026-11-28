import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.Dialog {
    id: graph

    required property var page
    property alias sources: body.sources
    property alias now: body.now
    property alias shown: body.shown
    property alias hidden: body.hidden
    property alias chosen: body.chosen
    readonly property alias ops: body.ops
    readonly property alias plot: body.plot
    readonly property alias body: body

    readonly property alias span: body.span
    readonly property alias pasteable: body.pasteable
    property alias aligned: body.aligned
    property alias linked: body.linked
    readonly property alias keyed: body.keyed
    readonly property alias compact: body.compact

    function load() {
        body.load();
    }

    function named(part) {
        return body.named(part);
    }

    function tint(s, part) {
        return body.tint(s, part);
    }

    function picked(s, t) {
        return body.picked(s, t);
    }

    function choose(s, t, adding) {
        body.choose(s, t, adding);
    }

    title: App.tr("Curve editor")
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 40
    onOpened: body.load()

    SceneGraphBody {
        id: body

        page: graph.page
        live: graph.visible
    }
}
