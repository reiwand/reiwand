import QtQuick
import QtWebEngine

Item {
    id: root

    property int viewId: 0
    property bool shown: true
    property alias web: web

    function load(address) {
        web.url = address;
    }

    function run(code) {
        web.runJavaScript(code);
    }

    function ping() {
        web.runJavaScript("0", 0, function () {
            bridge.answered(root.viewId);
        });
    }

    WebEngineView {
        id: web

        anchors.fill: parent
        profile: bridge.profile
        visible: root.shown
        audioMuted: bridge.muted
        backgroundColor: bridge.background
        settings.javascriptCanOpenWindows: false
        settings.javascriptCanAccessClipboard: false
        settings.localContentCanAccessFileUrls: false
        settings.localContentCanAccessRemoteUrls: false
        settings.playbackRequiresUserGesture: false
        settings.webRTCPublicInterfacesOnly: true
        settings.allowRunningInsecureContent: false
        settings.focusOnNavigationEnabled: false
        settings.errorPageEnabled: false
        settings.dnsPrefetchEnabled: false
        settings.pdfViewerEnabled: false
        settings.showScrollBars: false
        settings.screenCaptureEnabled: false
        settings.fullScreenSupportEnabled: false
        settings.webGLEnabled: true
        onLoadingChanged: function (request) {
            if (request.status === WebEngineView.LoadSucceededStatus)
                bridge.loaded(root.viewId);
        }
        onRenderProcessTerminated: function (status, code) {
            bridge.terminated(root.viewId, status);
        }
        onJavaScriptConsoleMessage: function (level, message, line, source) {
            bridge.console(root.viewId, message);
        }
        onNavigationRequested: function (request) {
            if (!request.url.toString().startsWith(bridge.own))
                request.action = WebEngineNavigationRequest.IgnoreRequest;
        }
        onNewWindowRequested: function (request) {}
        onJavaScriptDialogRequested: function (request) {
            request.accepted = true;
            request.dialogReject();
        }
        onFileDialogRequested: function (request) {
            request.accepted = true;
            request.dialogReject();
        }
        onPermissionRequested: function (permission) {
            permission.deny();
        }
        onFullScreenRequested: function (request) {}
    }
}
