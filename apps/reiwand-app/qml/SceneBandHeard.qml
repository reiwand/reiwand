import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.FormLayout {
    required property var picker

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Measured as")
        model: [App.tr("Relative to the music"), App.tr("Absolute"), App.tr("Surge over the usual")]
        currentIndex: ["relative", "absolute", "surge"].indexOf(picker.kind)
        onActivated: index => picker.kind = ["relative", "absolute", "surge"][index]
    }

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Scale")
        model: [App.tr("Decibels"), App.tr("Amplitude")]
        currentIndex: picker.unit === "amplitude" ? 1 : 0
        onActivated: index => picker.unit = index === 1 ? "amplitude" : "db"
    }

    QQC2.SpinBox {
        Kirigami.FormData.label: App.tr("Usual over (tenths of a second)")
        enabled: picker.kind === "surge"
        from: 1
        to: 600
        value: Math.round(picker.window * 10)
        onValueModified: picker.window = value / 10
    }

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Usual of")
        enabled: picker.kind === "surge"
        model: [App.tr("Its own band"), App.tr("All bands together")]
        currentIndex: picker.reference === "all" ? 1 : 0
        onActivated: index => picker.reference = index === 1 ? "all" : "band"
    }

    QQC2.CheckBox {
        text: App.tr("Keep the usual during silence")
        enabled: picker.kind === "surge"
        checked: picker.hold
        onToggled: picker.hold = checked
    }
}
