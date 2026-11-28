import QtQuick
import io.github.reiwand.app

Rectangle {
    function shade(alpha) {
        return Qt.rgba(Look.night.r, Look.night.g, Look.night.b, Math.max(0, Math.min(1, alpha)));
    }

    gradient: Gradient {
        GradientStop {
            position: 0
            color: shade(0)
        }

        GradientStop {
            position: 0.2
            color: shade(Look.veil * 0.72)
        }

        GradientStop {
            position: 0.38
            color: shade(0.56 + Look.veil * 0.4)
        }

        GradientStop {
            position: 1
            color: shade(0.62 + Look.veil * 0.36)
        }
    }
}
