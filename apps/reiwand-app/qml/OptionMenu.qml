import QtQuick
import io.github.reiwand.app

SmallPanel {
    id: menu

    property var options: []
    property int current: -1
    property int rowHeight: Look.optionHeight
    property real textSize: Look.tBody

    signal chosen(int index)

    function usable(index) {
        const option = options[index];
        return option !== undefined && !option.separator && !option.heading && !option.off;
    }

    function move(by) {
        let next = current;
        for (let tries = 0; tries < options.length; tries++) {
            next = (next + by + options.length) % options.length;
            if (usable(next))
                return current = next;
        }
    }

    function choose(index) {
        if (!usable(index))
            return;
        chosen(index);
        close();
    }

    width: Look.menuSmall
    padded: false
    first: list
    onAboutToShow: usable(current) || move(1)

    FocusScope {
        id: list

        width: menu.width
        implicitHeight: column.implicitHeight + 2 * Look.listGap
        Accessible.role: Accessible.PopupMenu
        Keys.onUpPressed: menu.move(-1)
        Keys.onDownPressed: menu.move(1)
        Keys.onReturnPressed: menu.choose(menu.current)
        Keys.onEnterPressed: menu.choose(menu.current)
        Keys.onSpacePressed: menu.choose(menu.current)

        Column {
            id: column

            x: Look.listGap
            y: Look.listGap
            width: parent.width - 2 * Look.listGap

            Repeater {
                model: menu.options

                Loader {
                    id: entry

                    required property var modelData
                    required property int index

                    width: column.width
                    sourceComponent: modelData.separator ? line : modelData.heading ? head : item

                    Component {
                        id: line

                        Item {
                            implicitHeight: Look.line + 2 * Look.listGap

                            Rectangle {
                                y: Look.listGap
                                width: parent.width
                                height: Look.line
                                color: Look.nightLine
                            }
                        }
                    }

                    // A heading names the entries under it; it cannot be chosen.
                    Component {
                        id: head

                        Text {
                            leftPadding: Look.e3 + Look.listGap
                            topPadding: Look.e1
                            bottomPadding: Look.e1
                            text: entry.modelData.heading
                            font.family: Look.textFont
                            font.pixelSize: menu.textSize - 1
                            font.weight: Font.DemiBold
                            color: Look.mistLow
                            Accessible.role: Accessible.Heading
                        }
                    }

                    Component {
                        id: item

                        OptionItem {
                            text: entry.modelData.text ?? ""
                            icon: entry.modelData.icon ?? ""
                            note: entry.modelData.note ?? ""
                            tone: entry.modelData.tone ?? ""
                            off: entry.modelData.off ?? false
                            plan: entry.modelData.plan ?? ""
                            current: entry.modelData.current ?? false
                            size: menu.textSize
                            high: menu.rowHeight
                            lit: list.activeFocus && menu.current === entry.index
                            onClicked: menu.choose(entry.index)
                        }
                    }
                }
            }
        }
    }
}
