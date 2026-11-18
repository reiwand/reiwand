import QtQuick
import io.github.reiwand.app

QtObject {
    id: motion

    required property var preview
    property string animated: ""
    property string playUid: ""

    function clear() {
        animated = "";
        playUid = "";
    }

    function look() {
        const uid = preview.candidate ? preview.candidate.uid : "";
        clear();
        if (uid === "")
            return;
        App.query("GetItem", [uid], "Library1", reply => motion && preview.candidate && preview.candidate.uid === uid && !(App.state.on_battery ?? false) && reply.about?.photosensitive !== true && motion.play(reply), () => {});
    }

    function play(reply) {
        playUid = ["scene", "video"].includes(reply.item?.package?.kind ?? reply.item?.kind) && !reply.item?.missing ? reply.item.uid : "";
        animated = App.url(reply.about?.animated ?? "");
    }
}
