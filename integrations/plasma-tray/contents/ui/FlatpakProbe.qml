import QtQuick
import org.kde.plasma.plasma5support as P5Support
import "flatpak.js" as Flatpak

// Runs the host commands of flatpak.js. Loaded through a Loader, so a Plasma without this module
// keeps the rest of the tray.
P5Support.DataSource {
    id: probe

    signal listed(string version)
    signal failed(string error)

    function check() {
        connectSource(Flatpak.listing());
    }

    function restart() {
        connectSource(Flatpak.restart());
    }

    engine: "executable"
    onNewData: (source, data) => {
        disconnectSource(source);
        if (source === Flatpak.listing())
            listed(Flatpak.installed(data.stdout));
        else if ((data["exit code"] ?? 0) !== 0)
            failed(String(data.stderr ?? "").trim());
    }
}
