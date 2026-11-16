import QtQuick
import io.github.reiwand.app

Aviso {
    id: adding

    readonly property var job: App.jobs.filter(job => job.kind === "account").reduce((last, job) => !last || job.id > last.id ? job : last, null)
    readonly property string said: job && job.error && job.error.message ? job.error.message : ""
    property var dismissed: null
    readonly property bool failed: !!job && job.state === "failed" && job.id !== dismissed

    text: Publishing.upload(said) === "red" ? App.tr("Could not reach the server. Check its address and that it is on, then try again.") : said || App.tr("The job failed without giving a reason")
    tone: "coral"
    onClosed: dismissed = job ? job.id : null
}
