import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

// A row of words and controls that fits a narrow panel: its words (labels, the words of check
// boxes) and its drop-down lists may get narrower than they would like, down to a floor, wrapping
// or eliding, instead of pushing the other controls past the panel's edge.
RowLayout {
    id: row

    function fitWords() {
        for (const item of children) {
            if (item.Layout.fillWidth)
                continue;
            const label = item.text !== undefined && item.wrapMode !== undefined && item.pressed === undefined;
            const check = item.checkable === true && item.text !== "" && item.display !== 0;
            const choice = item.currentIndex !== undefined && item.displayText !== undefined;
            if (!label && !check && !choice)
                continue;
            const wanted = item.Layout.preferredWidth > 0 ? item.Layout.preferredWidth : -1;
            if (label)
                item.wrapMode = Text.Wrap;
            item.Layout.fillWidth = true;
            item.Layout.minimumWidth = choice ? Look.e8 : Math.min(Look.e7, wanted > 0 ? wanted : Look.e7);
            item.Layout.maximumWidth = wanted > 0 ? wanted : Math.max(item.implicitWidth, Look.e8 * 4);
        }
    }

    Component.onCompleted: fitWords()
    onChildrenChanged: Qt.callLater(fitWords)
}
