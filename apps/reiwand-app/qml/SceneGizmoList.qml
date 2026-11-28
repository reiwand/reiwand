import QtQuick

QtObject {
    required property var editor
    property var list: []
    property var texts: []
    readonly property var blank: ({
        "id": "",
        "pointer": "",
        "offset": [0, 0],
        "position": [0, 0],
        "size": [0, 0],
        "scale": [1, 1],
        "skew": [0, 0],
        "origin": [0.5, 0.5],
        "rotation": 0,
        "tilt": [0, 0],
        "z": 0,
        "corners": null,
        "outline": [[0, 0], [0, 0], [0, 0], [0, 0]],
        "pivot": [0, 0],
        "angle": 0,
        "extent": [0, 0],
        "unit": [1, 0, 0, 1, 0, 0],
        "parent": null,
        "locked": true
    })

    function take() {
        const next = JSON.parse(editor.gizmos || "[]");
        const said = next.map(gizmo => JSON.stringify(gizmo));
        const kept = next.map((gizmo, i) => said[i] === texts[i] ? list[i] : gizmo);
        texts = said;
        if (kept.length !== list.length || kept.some((gizmo, i) => gizmo !== list[i]))
            list = kept;
    }

    Component.onCompleted: take()

    property Connections follow: Connections {
        target: editor

        function onGizmosChanged() {
            take();
        }
    }
}
