import QtQuick

Item {
    id: shown

    required property var page
    property string at
    property var fields: []

    signal moved

    function settle() {
        wait.stop();
        fields = JSON.parse(page.editor.fields || "[]");
        at = page.editor.selected;
        moved();
    }

    function follow() {
        if (page.editor.selected === at)
            settle();
        else
            wait.restart();
    }

    visible: false
    Component.onCompleted: settle()

    Timer {
        id: wait

        interval: 50
        onTriggered: shown.settle()
    }

    Connections {
        target: shown.page.editor

        function onFieldsChanged() {
            shown.follow();
        }

        function onSelectedChanged() {
            shown.follow();
        }
    }

    Connections {
        target: shown.page.Window.window
        enabled: wait.running

        function onFrameSwapped() {
            shown.settle();
        }
    }
}
