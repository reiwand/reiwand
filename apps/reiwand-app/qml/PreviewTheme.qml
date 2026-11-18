import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

QtObject {
    required property var host

    Component.onCompleted: {
        const theme = host.Kirigami.Theme;
        theme.backgroundColor = Qt.binding(() => Look.noche);
        theme.alternateBackgroundColor = Qt.binding(() => Look.nocheAlta);
        theme.textColor = Qt.binding(() => Look.escarcha);
        theme.disabledTextColor = Qt.binding(() => Look.brumaBaja);
        theme.highlightColor = Qt.binding(() => Look.luna);
        theme.highlightedTextColor = Qt.binding(() => Look.noche);
        theme.focusColor = Look.escarcha;
        theme.hoverColor = Qt.binding(() => Look.bruma);
    }
}
