import QtQuick
import io.github.reiwand.app

SceneMenu {
    id: menu

    required property var timeline
    required property var bar

    function onKey(track, index, item, x, y) {
        timeline.chosen = {
            "pointer": track.pointer,
            "index": index
        };
        show(item, x, y, [
            {
                "text": App.tr("Curve and base of the keyframe"),
                "icon": "draw-bezier-curves",
                "act": () => menu.bar.curve()
            },
            {
                "separator": true
            },
            {
                "text": App.tr("Remove the keyframe"),
                "icon": "edit-delete",
                "tone": "coral",
                "act": () => {
                    menu.timeline.edit(undefined, true);
                    menu.timeline.chosen = null;
                }
            }
        ]);
    }

    function onTrack(track, item, x, y) {
        show(item, x, y, [
            {
                "text": App.tr("Show in the inspector"),
                "icon": "document-properties",
                "act": () => menu.timeline.page.jump(track.pointer)
            }
        ]);
    }
}
