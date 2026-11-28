import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

QtObject {
    required property var host

    Component.onCompleted: {
        const theme = host.Kirigami.Theme;
        theme.backgroundColor = Qt.binding(() => Look.night);
        theme.alternateBackgroundColor = Qt.binding(() => Look.nightHigh);
        theme.textColor = Qt.binding(() => Look.frost);
        theme.disabledTextColor = Qt.binding(() => Look.mistLow);
        theme.highlightColor = Qt.binding(() => Look.moon);
        theme.highlightedTextColor = Qt.binding(() => Look.night);
        theme.focusColor = Look.frost;
        theme.hoverColor = Qt.binding(() => Look.mist);
    }
}
