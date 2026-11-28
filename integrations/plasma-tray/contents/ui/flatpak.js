.pragma library

// After `flatpak update` the service and this tray keep running the version they started with; only
// the host knows a newer one is installed. These are the host commands the tray runs for that.

const APP = "io.github.reiwand.Reiwand";

// One line per installed app, "id<TAB>version"; with no Flatpak (or none of ours) it prints nothing.
function listing() {
    return "flatpak list --app --columns=application,version";
}

function installed(stdout) {
    for (const line of String(stdout ?? "").split("\n")) {
        const fields = line.trim().split(/\s+/);
        if (fields[0] === APP)
            return fields[1] ?? "";
    }
    return "";
}

function bare(version) {
    return String(version ?? "").split("+")[0];
}

// A newer Flatpak is installed while the service still runs another version (a build of one's own,
// "-local", is never compared).
function newer(installedVersion, running) {
    const [mine, now] = [bare(installedVersion), bare(running)];
    return mine !== "" && now !== "" && !now.endsWith("-local") && mine !== now;
}

// The processes of the old service: only reiwandd, since `flatpak kill` would also end the app and
// lose what its editor has not kept yet. The shell prints their host process ids.
function services() {
    return "flatpak ps --columns=child-pid,application | while read p a; do [ \"$a\" = " + APP + " ] && [ \"$(cat /proc/$p/comm 2>/dev/null)\" = reiwandd ] && echo $p; done";
}

// The guided restart, outside Plasma so it outlives it: the new version copies its wallpaper
// plugin and tray into Plasma, the old service stops (asked to, and waited for up to 8 seconds, so
// the new one can take its name), and Plasma starts again (which starts the new service when the
// wallpaper asks for it).
function restart() {
    const stop = "for p in $(" + services() + "); do kill $p; n=0; while kill -0 $p 2>/dev/null && [ $n -lt 40 ]; do sleep 0.2; n=$((n+1)); done; done";
    return "systemd-run --user --collect --quiet sh -c 'flatpak run --command=reiwand-plasma " + APP + " | sh; " + stop + "; systemctl --user restart plasma-plasmashell.service'";
}
