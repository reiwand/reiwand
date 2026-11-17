import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: tasks

    property var dismissed: []
    readonly property var shown: pick(App.jobs)

    function pick(jobs) {
        return jobs.filter(job => job.kind !== "layers").filter(job => job.state === "queued" || job.state === "running" || (job.state === "failed" && dismissed.indexOf(job.id) < 0 && !jobs.some(later => later.id > job.id && later.kind === job.kind && later.target === job.target)));
    }

    spacing: Look.e1
    visible: shown.length > 0

    Repeater {
        model: tasks.shown

        delegate: Row {
            id: task

            required property var modelData
            readonly property bool failed: modelData.state === "failed"
            readonly property string what: modelData.kind === "analyze" ? App.tr("Making the thumbnails") : Labels.value(modelData.kind)
            readonly property string said: (modelData.error && modelData.error.message) || ""
            readonly property bool unreachable: ["account", "upload"].includes(modelData.kind) && Publishing.upload(said) === "red"

            anchors.right: parent ? parent.right : undefined
            spacing: Look.e2

            Estado {
                anchors.verticalCenter: parent.verticalCenter
                icon: task.failed ? "warning-circle" : "images"
                tone: task.failed ? "coral" : ""
                text: task.failed ? task.what + ": " + (task.unreachable ? App.tr("Could not reach the server. Check its address and that it is on, then try again.") : task.said || App.tr("The job failed without giving a reason")) : task.modelData.total > 0 ? App.tr("%1: %2 of %3", task.what, task.modelData.done, task.modelData.total) : task.what
            }

            QQC2.Button {
                anchors.verticalCenter: parent.verticalCenter
                flat: true
                text: task.failed ? App.tr("Close") : App.tr("Cancel")
                onClicked: task.failed ? tasks.dismissed = tasks.dismissed.concat([task.modelData.id]) : App.act("CancelJob", [task.modelData.id])
            }
        }
    }
}
