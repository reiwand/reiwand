import QtQuick
import QtMultimedia

Item {
    id: clip

    property url source
    readonly property bool running: player.playbackState === MediaPlayer.PlayingState && out.videoSink.videoFrame.isValid

    MediaPlayer {
        id: player

        source: clip.source
        loops: MediaPlayer.Infinite
        videoOutput: out
        Component.onCompleted: play()
    }

    VideoOutput {
        id: out

        anchors.fill: parent
        fillMode: VideoOutput.PreserveAspectCrop
    }
}
