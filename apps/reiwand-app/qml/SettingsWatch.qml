import QtQuick
import io.github.reiwand.app

Connections {
    required property var owner

    target: App.backend

    function onChanged(topic) {
        if (topic === "library" || topic === "inputs")
            owner.load();
    }
}
