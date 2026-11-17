import QtQuick
import io.github.reiwand.app

Item {
    id: reel

    property real aspect: 16 / 9
    property real unit: 1
    property bool calm: false
    property string nowUid: ""
    property string animated: ""
    property alias cards: cards
    signal chosen(int offset)

    function slots(state) {
        const found = [];
        const behind = state.behind ?? [];
        behind.forEach((item, at) => found.push([item, -(at + 1)]));
        if (state.candidate)
            found.push([state.candidate, 0]);
        (state.ahead ?? []).forEach((item, at) => found.push([item, at + 1]));
        return found;
    }

    function has(uid) {
        for (let at = 0; at < cards.count; at++)
            if (cards.get(at).uid === uid)
                return true;
        return false;
    }

    function sync(state) {
        const wanted = slots(state);
        for (let at = cards.count - 1; at >= 0; at--) {
            const row = cards.get(at);
            const slot = wanted.find(w => w[0].uid === row.uid);
            if (slot)
                cards.set(at, {
                    "json": JSON.stringify(slot[0]),
                    "offset": slot[1],
                    "gone": false
                });
            else
                cards.set(at, {
                    "offset": row.offset < 0 ? -3 : 3,
                    "gone": true
                });
        }
        for (const slot of wanted)
            if (!has(slot[0].uid))
                cards.append({
                    "uid": slot[0].uid,
                    "json": JSON.stringify(slot[0]),
                    "offset": slot[1],
                    "gone": false
                });
        sweep.restart();
    }

    function card(offset) {
        for (let at = 0; at < strip.children.length; at++)
            if (strip.children[at].offset === offset && !strip.children[at].gone)
                return strip.children[at];
        return null;
    }

    implicitHeight: Math.round(unit * PreviewLook.focoAncho / aspect)
    Accessible.role: Accessible.List
    Accessible.name: App.tr("Wallpapers")

    ListModel {
        id: cards
    }

    Timer {
        id: sweep

        interval: reel.calm ? 1 : PreviewLook.paso + 40
        onTriggered: {
            for (let at = cards.count - 1; at >= 0; at--)
                if (cards.get(at).gone)
                    cards.remove(at);
        }
    }

    Item {
        id: strip

        anchors.fill: parent

        Repeater {
            model: cards

            delegate: PreviewCard {
                aspect: reel.aspect
                unit: reel.unit
                calm: reel.calm
                home: strip.width / 2
                current: uid === reel.nowUid
                animated: offset === 0 ? reel.animated : ""
                onClicked: offset => reel.chosen(offset)
            }
        }
    }
}
