import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

ColumnLayout {
    id: cache

    required property var page
    property string kind
    property var rows: []

    signal shown(string kind)

    function show(next, list) {
        kind = next;
        rows = list;
        let found = false;
        for (let i = 0; i < seen.count; i++)
            found = found || seen.get(i).kind === next;
        if (!found)
            seen.append({
                "kind": next
            });
        shown(next);
    }

    spacing: Look.e1

    ListModel {
        id: seen
    }

    Repeater {
        model: seen

        delegate: SceneFieldList {
            id: list

            required property string kind

            Layout.fillWidth: true
            visible: kind === cache.kind
            page: cache.page
            Component.onCompleted: rows = kind === cache.kind ? cache.rows : []

            Connections {
                target: cache

                function onShown(kind) {
                    if (kind === list.kind)
                        list.rows = cache.rows;
                }
            }
        }
    }
}
