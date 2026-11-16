import QtQuick

SceneFieldList {
    id: held

    function take() {
        const now = page.shown;
        if (!(page.view.layers ?? []).some(item => item.pointer === now.at))
            rows = now.fields;
    }

    Component.onCompleted: take()

    Connections {
        target: held.page.shown

        function onMoved() {
            held.take();
        }
    }
}
