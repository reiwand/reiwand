import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Item {
    id: pulls

    required property var swarm
    readonly property real r: swarm.ratio
    readonly property var list: (swarm.info?.pulls ?? []).filter(e => Array.isArray(e.at))

    function patch(e, change) {
        const v = swarm.forceOf(e.index);
        change(v[e.kind]);
        swarm.edit("force", {
                "index": e.index,
                "force": v
            });
    }

    function reach(e, ring, to) {
        const unit = ring.value > 0 ? ring.px / ring.value : 1;
        const d = Math.round(Math.hypot(to[0] - e.at[0], to[1] - e.at[1]) / Math.max(1e-6, unit) * 10) / 10;
        patch(e, v => {
                if (ring.key === "ring")
                    v.ring.radius = d;
                else
                    v[ring.key] = d;
            });
    }

    onListChanged: art.requestPaint()
    onRChanged: art.requestPaint()

    Canvas {
        id: art

        anchors.fill: parent
        onPaint: {
            const ctx = getContext("2d"), r = pulls.r;
            ctx.reset();
            ctx.strokeStyle = Look.amber;
            ctx.lineWidth = 1.5;
            ctx.setLineDash([4, 3]);
            pulls.list.forEach(e => e.rings.forEach(ring => {
                            ctx.beginPath();
                            ctx.arc(e.at[0] * r, e.at[1] * r, ring.px * r, 0, 2 * Math.PI);
                            ctx.stroke();
                        }));
        }
    }

    readonly property real spoke: 80

    function tip(e) {
        const turn = e.angles[1] * Math.PI / 180;
        const far = pulls.spoke * (0.3 + 0.7 * Math.sin(e.angles[0] * Math.PI / 180));
        return [e.at[0] + far * Math.cos(turn), e.at[1] + far * Math.sin(turn)];
    }

    function aim(e, to) {
        const dx = to[0] - e.at[0], dy = to[1] - e.at[1];
        const lean = Math.min(1, Math.max(0, (Math.hypot(dx, dy) / pulls.spoke - 0.3) / 0.7));
        swarm.edit("aim", {
                "target": "force:" + e.index,
                "tilt": Math.round(Math.asin(lean) * 1800 / Math.PI) / 10,
                "turn": Math.round(Math.atan2(dy, dx) * 1800 / Math.PI) / 10
            });
    }

    SceneRepeater {
        id: listed2

        rows: pulls.list

        delegate: Item {
            id: pull

            required property int index
            readonly property var modelData: listed2.rows[index]

            anchors.fill: parent

            SceneKnob {
                at: pull.modelData.at
                ratio: pulls.r
                label: SceneWords.value(pull.modelData.kind)
                tint: Look.amber
                tip: App.tr("Drag to move the center of the force")
                onMoved: (to, done) => SceneSwarmGrips.spot(pulls, pull.modelData, pull.modelData.ref, to, (v, p) => v[pull.modelData.field] = p)
            }

            SceneKnob {
                visible: pull.modelData.kind === "vortex"
                at: visible ? pulls.tip(pull.modelData) : pull.modelData.at
                ratio: pulls.r
                label: App.tr("Axis")
                tint: Look.amber
                tip: App.tr("Drag to aim the axis the particles turn around: the direction sets the turn, the distance from the center sets the tilt")
                onMoved: (to, done) => pulls.aim(pull.modelData, to)
            }

            SceneRepeater {
                id: listed

                rows: pull.modelData.rings

                delegate: SceneKnob {
                    required property int index
                    readonly property var modelData: listed.rows[index]

                    at: modelData.at
                    ratio: pulls.r
                    square: true
                    label: SceneWords.label(modelData.key)
                    tint: Look.amber
                    tip: App.tr("Drag to change how far the force reaches")
                    onMoved: (to, done) => pulls.reach(pull.modelData, modelData, to)
                }
            }
        }
    }
}
