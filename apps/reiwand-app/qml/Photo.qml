import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Image {
    id: photo

    property real corner: Look.radius
    property real lowCorner: corner

    asynchronous: true
    cache: true
    fillMode: Image.PreserveAspectCrop
    layer.enabled: status === Image.Ready && (corner > 0 || lowCorner > 0) && GraphicsInfo.api !== GraphicsInfo.Software && GraphicsInfo.api !== GraphicsInfo.Unknown
    layer.effect: Kirigami.ShadowedTexture {
        color: "transparent"
        radius: photo.corner
        corners.bottomLeftRadius: photo.lowCorner
        corners.bottomRightRadius: photo.lowCorner
    }
}
