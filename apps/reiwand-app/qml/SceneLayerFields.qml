import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: groups

    required property var page
    required property var node
    readonly property var everyday: ["transform", "size", "opacity", "visible", "anchor"]
    readonly property var sections: [["blend", App.tr("Color and blending"), ["blend", "blend_space", "brightness", "lighting", "solid", "cache"]], ["depth", App.tr("Depth"), ["parallax", "depth_test", "attach", "inherit", "parent"]], ["mask", App.tr("Mask"), ["mask"]]]
    readonly property var kinds: ({
            "image": App.tr("Image and crop"),
            "text": App.tr("Lettering and color"),
            "shape": App.tr("Shape and fill"),
            "particles": App.tr("Particles"),
            "light": App.tr("Light"),
            "model": App.tr("Model and materials"),
            "mesh": App.tr("Mesh"),
            "rig": App.tr("Rig and bones"),
            "touch": App.tr("Touch and sensors")
        })
    property var held: ({})
    property var fields: []
    readonly property var placed: [].concat(everyday, ["effects", "id", "type"], sections.reduce((all, section) => all.concat(section[2]), []))

    function head(field) {
        if (!held.pointer)
            return "";
        return field.pointer.replace(/^\/scene(\/layers\/\d+)+\//, "").split("/")[0];
    }

    function hold() {
        const now = page.shown;
        if (!node.pointer || node.pointer !== now.at)
            return;
        fields = now.fields;
        held = node;
        daily.show(node.type ?? "", rows(everyday));
    }

    function rows(keys) {
        return groups.fields.filter(field => keys.includes(head(field)));
    }

    spacing: Look.e1
    onNodeChanged: hold()
    Component.onCompleted: hold()

    Connections {
        target: groups.page.shown

        function onMoved() {
            groups.hold();
        }
    }

    SceneKindRows {
        id: daily

        Layout.fillWidth: true
        page: groups.page
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        Accessible.name: App.tr("Moves on its own")
        enabled: !groups.held.locked
        displayText: App.tr("Moves on its own: %1", App.tr("No"))
        model: starts.moves.map(move => move[2])
        onActivated: index => starts.keepMoving(starts.moves[index])

        SceneLayerStarts {
            id: starts

            owner: ({
                    "page": groups.page,
                    "current": groups.held
                })
        }
    }

    // While the workbench has the Effects panel, the layer's effects are there.
    SceneEffectsCard {
        Layout.fillWidth: true
        visible: groups.held.type !== "sound" && !groups.page.layout?.groupOf("effects")
        page: groups.page
        node: groups.held
        rows: groups.rows(["effects"])
    }

    SceneFold {
        id: kindFold

        readonly property var shown: groups.held.pointer ? groups.fields.filter(field => !groups.placed.includes(groups.head(field))) : []

        Layout.fillWidth: true
        visible: shown.length > 0
        key: "layer-kind"
        title: groups.kinds[groups.held.type] ?? App.tr("Content")
        open: false

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: kindFold.reached ? kindFold.shown : []
        }
    }

    SceneFold {
        id: blendFold

        Layout.fillWidth: true
        visible: groups.rows(groups.sections[0][2]).length > 0
        key: "layer-blend"
        title: groups.sections[0][1]
        open: false

        SceneLayerLook {
            Layout.fillWidth: true
            page: groups.page
            node: groups.held
        }

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: blendFold.reached ? groups.rows(groups.sections[0][2]).filter(field => field.key !== "blend" && field.key !== "blend_space") : []
        }
    }

    SceneFold {
        id: depthFold

        Layout.fillWidth: true
        visible: groups.rows(groups.sections[1][2]).length > 0
        key: "layer-depth"
        title: groups.sections[1][1]
        open: false

        SceneLayerDepth {
            Layout.fillWidth: true
            page: groups.page
            node: groups.held
            pointer: groups.held.pointer ?? ""
        }

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: depthFold.reached ? groups.rows(groups.sections[1][2]) : []
        }
    }

    SceneFold {
        id: maskFold

        Layout.fillWidth: true
        visible: groups.rows(groups.sections[2][2]).length > 0
        key: "layer-mask"
        title: groups.sections[2][1]
        open: false

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: maskFold.reached ? groups.rows(groups.sections[2][2]) : []
        }
    }
}
