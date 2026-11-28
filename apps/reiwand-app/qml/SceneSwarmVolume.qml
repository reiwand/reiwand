import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

SceneGrid {
    id: volume

    required property var tools
    required property var emitter
    readonly property var names: [App.tr("Horizontal: both sides"), App.tr("Horizontal: right side"), App.tr("Horizontal: left side"), App.tr("Vertical: both sides"), App.tr("Vertical: lower side"), App.tr("Vertical: upper side"), App.tr("Depth: both sides"), App.tr("Depth: far side"), App.tr("Depth: near side")]
    readonly property real reach: Math.max(1, Math.abs(emitter.depth ?? 0) + (emitter.radius?.[2] ?? 0) + (emitter.push ?? 0), (emitter.radius?.[0] ?? 0) + (emitter.push ?? 0))

    function shape(change) {
        const s = {
            "center": emitter.local,
            "depth": emitter.depth,
            "radius": emitter.radius.slice(),
            "inner": emitter.inner,
            "sign": emitter.sign.slice()
        };
        if ((emitter.push ?? 0) > 0)
            s.push = emitter.push;
        if (typeof emitter.anchor === "string")
            s.at = emitter.anchor;
        change(s);
        if (s.push === 0)
            delete s.push;
        return {
            "sphere": s
        };
    }

    function commit(change, drag) {
        tools.edit("shape", {
                "emitter": emitter.index,
                "shape": shape(change)
            }, drag === true);
    }


    QQC2.Label {
        Layout.columnSpan: 2
        text: App.tr("Side view of the ball")
        font.bold: true
    }

    Canvas {
        id: side

        objectName: "ballSide"
        Layout.columnSpan: 2
        Layout.fillWidth: true
        Layout.preferredHeight: 140
        readonly property real unit: Math.min(width / 2, height / 2) / volume.reach * 0.85
        readonly property real mid: height / 2

        function depthOf(y) {
            return (y - mid) / unit;
        }

        onPaint: {
            const ctx = getContext("2d"), e = volume.emitter, u = unit;
            if (!e?.radius)
                return;
            ctx.reset();
            ctx.strokeStyle = Kirigami.Theme.textColor;
            ctx.globalAlpha = 0.4;
            ctx.beginPath();
            ctx.moveTo(0, mid);
            ctx.lineTo(width, mid);
            ctx.stroke();
            ctx.globalAlpha = 1;
            ctx.strokeStyle = Kirigami.Theme.highlightColor;
            ctx.fillStyle = Kirigami.Theme.highlightColor;
            ctx.lineWidth = 2;
            const cy = mid + (e.depth ?? 0) * u;
            const push = e.push ?? 0, deep = e.radius[2] > 0 ? push : 0;
            [1, e.inner ?? 0].filter((k, i) => !i || k > 0 || push <= 0).forEach((k, i) => {
                const w = (e.radius[0] * k + push) * u, h = (e.radius[2] * k + deep) * u;
                ctx.setLineDash(i ? [4, 4] : []);
                ctx.beginPath();
                ctx.ellipse(width / 2 - w, cy - h, 2 * w, 2 * h);
                ctx.stroke();
            });
            ctx.setLineDash([]);
            [[width / 2, cy], [width / 2, cy + (e.radius[2] + push) * u]].forEach(p => {
                ctx.beginPath();
                ctx.arc(p[0], p[1], 5, 0, 2 * Math.PI);
                ctx.fill();
            });
        }

        MouseArea {
            property bool edge: false

            anchors.fill: parent
            onPressed: mouse => {
                const e = volume.emitter;
                if (!e?.radius)
                    return;
                const cy = side.mid + (e.depth ?? 0) * side.unit;
                edge = Math.abs(mouse.y - (cy + (e.radius[2] + (e.push ?? 0)) * side.unit)) < Math.abs(mouse.y - cy);
            }
            onPositionChanged: mouse => {
                const z = side.depthOf(mouse.y);
                volume.commit(s => {
                    if (edge)
                        s.radius[2] = Math.max(0, Math.round((z - s.depth - (s.push ?? 0)) * 10) / 10);
                    else
                        s.depth = Math.round(z);
                }, true);
            }
            onReleased: volume.commit(s => {})
        }

        Connections {
            function onEmitterChanged() {
                side.requestPaint();
            }

            target: volume
        }

        Component.onCompleted: requestPaint()
    }

    QQC2.Label {
        text: App.tr("Depth radius")
    }

    SceneSlider {
        objectName: "ballDepthRadius"
        Layout.fillWidth: true
        from: 0
        to: 2000
        value: volume.emitter.radius?.[2] ?? 0
        onMoved: volume.commit(s => s.radius[2] = Math.round(value), true)
    }

    QQC2.Label {
        text: App.tr("Hollow core")
    }

    SceneSlider {
        objectName: "ballInner"
        Layout.fillWidth: true
        from: 0
        to: 1
        value: volume.emitter.inner ?? 0
        onMoved: volume.commit(s => s.inner = Math.round(value * 100) / 100, true)
    }

    QQC2.Label {
        text: App.tr("Push")
    }

    SceneSlider {
        objectName: "ballPush"
        Layout.fillWidth: true
        from: 0
        to: 2000
        value: volume.emitter.push ?? 0
        onMoved: volume.commit(s => s.push = Math.round(value), true)
    }

    Repeater {
        model: 3

        delegate: QQC2.Button {
            required property int index

            objectName: "ballSign" + index
            Layout.columnSpan: 2
            Layout.fillWidth: true
            text: volume.names[index * 3 + [0, 1, -1].indexOf(volume.emitter.sign?.[index] ?? 0)]
            onClicked: volume.commit(s => s.sign[index] = [1, -1, 0][[0, 1, -1].indexOf(s.sign[index])])
        }
    }
}
