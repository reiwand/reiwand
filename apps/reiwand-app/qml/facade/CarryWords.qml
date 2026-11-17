pragma Singleton

import QtQuick
import io.github.reiwand.app

QtObject {
    function size(bytes) {
        if (bytes >= 1073741824)
            return App.tr("%1 GB", (bytes / 1073741824).toFixed(1));
        if (bytes >= 1048576)
            return App.tr("%1 MB", Math.round(bytes / 1048576));
        return App.tr("%1 KB", Math.max(1, Math.round(bytes / 1024)));
    }

    function section(name) {
        return ({
                "repositories": App.tr("Repositories"),
                "providers": App.tr("Photo sources"),
                "lists": App.tr("Lists"),
                "favourites": App.tr("Favorites"),
                "ratings": App.tr("Ratings"),
                "tags": App.tr("Tags"),
                "installed": App.tr("Wallpapers from repositories"),
                "settings": App.tr("Settings"),
                "own": App.tr("My own wallpapers"),
                "secrets": App.tr("Passwords and keys")
            })[name] ?? name;
    }

    function counts(row) {
        const parts = [];
        if (row.new > 0)
            parts.push(App.tr("%1 new", row.new));
        if (row.skipped > 0)
            parts.push(App.tr("%1 already here", row.skipped));
        if (row.conflicts > 0)
            parts.push(App.tr("%1 with the same name", row.conflicts));
        return parts.length > 0 ? parts.join(", ") : App.tr("Nothing to bring");
    }

    function entries(rows) {
        return rows.map(row => ({
                    "section": row.section,
                    "label": section(row.section),
                    "detail": counts(row),
                    "conflicts": row.conflicts,
                    "checked": row.section !== "settings"
                }));
    }

    function pending(item) {
        const reason = String(item.reason ?? "");
        if (item.section === "repositories") {
            if (/certificate|fingerprint/i.test(reason))
                return {
                    "label": App.tr("Confirm the certificate"),
                    "page": 8
                };
            if (/account|sign|private/i.test(reason))
                return {
                    "label": App.tr("Sign in again"),
                    "page": 6
                };
            return {
                "label": App.tr("Open the repositories"),
                "page": 8
            };
        }
        if (item.section === "providers" || item.section === "secrets")
            return {
                "label": App.tr("Open the photo sources"),
                "page": 3
            };
        return null;
    }
}
