import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Binding {
    required property var host
    property bool tight: false

    target: Look
    property: "tier"
    value: tight && Look.step(host.width, host.height) !== "short" ? "compact" : Look.step(host.width, host.height)
    Component.onCompleted: {
        const theme = host.Kirigami.Theme;
        theme.negativeTextColor = Look.coral;
        theme.neutralTextColor = Look.amber;
        theme.positiveTextColor = Look.sprout;
        theme.linkColor = Look.moon;
        theme.highlightColor = Qt.binding(() => Look.moon);
        theme.highlightedTextColor = Qt.binding(() => Look.night);
        theme.focusColor = Look.frost;
        theme.hoverColor = Qt.binding(() => Look.mist);
        theme.activeTextColor = Qt.binding(() => Look.moon);
    }
}
