import QtQuick
import io.github.reiwand.app

Item {
    id: scene

    readonly property alias copying: copying

    function start(uid) {
        App.query("GetItem", [uid], "Library1", reply => scene && scene.got(reply.item ?? {}), () => {});
    }

    function got(item) {
        const path = String(item.path ?? "");
        if (path.endsWith(".wpak") && item.missing !== true)
            copying.start(path);
        else
            applicationWindow().showPassiveNotification({
                "text": App.tr("Only a scene kept as a package can be copied from here."),
                "tone": "coral"
            }, 6000);
    }

    ItemCopy {
        id: copying
    }
}
