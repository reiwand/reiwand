import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Canvas {
    id: plot

    property var wave: null

    implicitHeight: Kirigami.Units.gridUnit * 4
    onWaveChanged: requestPaint()
    onWidthChanged: requestPaint()
    onPaint: {
        const ctx = getContext("2d");
        ctx.reset();
        ctx.strokeStyle = Kirigami.Theme.disabledTextColor;
        ctx.strokeRect(0, 0, width, height);
        if (!wave)
            return;
        const all = wave.curves.reduce((a, c) => a.concat(c), []);
        const low = Math.min(...all, 0), high = Math.max(...all, 1);
        const y = v => (1 - (v - low) / Math.max(1e-9, high - low)) * (height - 4) + 2;
        const one = y(1);
        ctx.setLineDash([3, 3]);
        ctx.beginPath();
        ctx.moveTo(0, one);
        ctx.lineTo(width, one);
        ctx.stroke();
        ctx.setLineDash([]);
        const tones = [Kirigami.Theme.disabledTextColor, Kirigami.Theme.highlightColor, Look.ambar];
        wave.curves.forEach((curve, k) => {
            ctx.strokeStyle = tones[k];
            ctx.lineWidth = k === 0 ? 1 : 2;
            ctx.beginPath();
            curve.forEach((v, i) => {
                const x = i / (curve.length - 1) * width;
                i ? ctx.lineTo(x, y(v)) : ctx.moveTo(x, y(v));
            });
            ctx.stroke();
        });
    }
}
