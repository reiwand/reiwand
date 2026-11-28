import QtQuick
import io.github.reiwand.app

// The Review panel: the path to publishing inside the workbench, its three steps one under the other
// (check the package and answer its questions, sign it, choose where it goes and publish), the same
// steps that had a page of their own. Publish… brings the Wallpaper layout with this panel in sight.
Rectangle {
    id: review

    required property var page
    readonly property var builder: page.builder

    function bring() {
        const layout = page.layout;
        if (layout.parsed.chosen.factory !== "wallpaper")
            layout.chooseKey(layout.kits.indexOf("wallpaper") + 1);
        layout.reveal("review");
    }

    color: Look.night
    clip: true
    Component.onCompleted: builder.host = review
    Component.onDestruction: {
        if (builder && builder.host === review)
            builder.host = null;
    }
    onVisibleChanged: {
        if (!visible)
            return;
        builder.reload();
        builder.sign.refresh();
        builder.review.refresh();
    }
}
