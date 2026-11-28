import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.ScrollView {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)

    // The keyboard is followed: a control that takes it inside this scroll (and no deeper one) is
    // brought into view with its focus ring.
    function reveal(item) {
        const flick = control.contentItem;
        if (!item || !flick || flick.contentY === undefined)
            return;
        let near = item.parent;
        while (near && near.contentY === undefined)
            near = near.parent;
        if (near !== flick)
            return;
        const top = item.mapToItem(flick, 0, 0).y - Look.ringRoom, bottom = top + item.height + 2 * Look.ringRoom;
        const shift = top < 0 || bottom - top > flick.height ? top : Math.max(0, bottom - flick.height);
        const least = flick.originY - flick.topMargin, most = flick.originY + flick.contentHeight + flick.bottomMargin - flick.height;
        if (shift !== 0)
            flick.contentY = Math.max(least, Math.min(Math.max(least, most), flick.contentY + shift));
    }

    // Held in a property: an object declared inside a ScrollView becomes its content, and with two
    // pieces of content it no longer takes a ListView as its own flickable.
    readonly property Connections follower: Connections {
        target: control.Window.window

        function onActiveFocusItemChanged() {
            control.reveal(control.Window.activeFocusItem);
        }
    }

    T.ScrollBar.vertical: ScrollBar {
        id: upright

        parent: control
        x: control.mirrored ? 0 : control.width - width
        y: control.topPadding
        height: control.availableHeight
        active: level.active
    }

    T.ScrollBar.horizontal: ScrollBar {
        id: level

        parent: control
        x: control.leftPadding
        y: control.height - height
        width: control.availableWidth
        active: upright.active
    }
}
