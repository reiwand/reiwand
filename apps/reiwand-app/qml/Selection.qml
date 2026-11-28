import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

RowLayout {
    id: selection

    property int count
    default property alias actions: tail.data
    readonly property var widths: {
        const all = [];
        for (let i = 0; i < tail.children.length; i++) {
            const child = tail.children[i];
            if (child.visible)
                all.push(child.implicitWidth);
        }
        return all;
    }
    readonly property real line: widths.reduce((sum, width) => sum + width, 0) + Math.max(0, widths.length - 1) * tail.columnSpacing

    function fits(columns) {
        let sum = (columns - 1) * tail.columnSpacing;
        for (let column = 0; column < columns; column++) {
            let widest = 0;
            for (let i = column; i < widths.length; i += columns)
                widest = Math.max(widest, widths[i]);
            sum += widest;
        }
        return sum <= tail.width + 0.5;
    }

    spacing: Look.e3
    Accessible.role: Accessible.Grouping
    Accessible.name: chosen.text

    Text {
        id: chosen

        rightPadding: Look.e2
        text: selection.count === 1 ? App.trc("covers", "1 chosen") : App.trc("covers", "%1 chosen", selection.count)
        font.family: Look.titleFont
        font.pixelSize: Look.tFocus
        font.weight: Font.Bold
        font.letterSpacing: Look.tFocus * Look.tight
        color: Look.frost
    }

    GridLayout {
        id: tail

        Layout.preferredWidth: selection.line
        Layout.minimumWidth: Math.max(0, ...selection.widths)
        columns: {
            for (let columns = Math.max(1, selection.widths.length); columns > 1; columns--) {
                if (selection.fits(columns))
                    return columns;
            }
            return 1;
        }
        columnSpacing: Look.e3
        rowSpacing: Look.e2
    }
}
