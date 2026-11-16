import QtQuick
import io.github.reiwand.app

QtObject {
    id: still

    property var told: ({})
    readonly property var outputs: App.state.outputs ?? {}
    readonly property string heavy: App.outputs.find(name => {
        const shown = still.outputs[name] ?? {};
        return shown.quality_auto === "still" && (shown.drafted === true ? SceneLook.viewed !== "" : !!shown.item);
    }) ?? ""

    function tell(output) {
        const shown = outputs[output];
        const mark = output + "/" + (shown.drafted ? "draft/" + SceneLook.viewed : shown.item);
        if (told[mark])
            return;
        told[mark] = true;
        if (shown.drafted)
            return say(output, SceneLook.viewed);
        App.query("GetItem", [shown.item], "Library1", reply => still && still.say(output, Names.of(reply.item)));
    }

    function say(output, title) {
        applicationWindow().showPassiveNotification({
            "text": App.tr("“%1” stays still on its poster. It is too heavy for this computer with the automatic quality. If you play it anyway it uses the lowest quality: it may drop frames and use more battery.", title),
            "tone": "ambar"
        }, 15000, App.tr("Play anyway"), () => App.act("SetOption", ["playback.quality." + output, "min-q1"]));
    }

    onHeavyChanged: heavy !== "" && tell(heavy)
}
