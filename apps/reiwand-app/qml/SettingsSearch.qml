import QtQuick
import io.github.reiwand.app

SearchBox {
    id: search

    required property var owner

    placeholder: App.tr("Search a setting")
    text: owner.query
    onTextChanged: owner.query !== text && (owner.query = text)
    // Enter goes to the first setting found, as the bar says; Down only moves to the results.
    onAccepted: {
        if (owner.found.hits.length === 0)
            return owner.found.enter();
        owner.reach(owner.found.hits[0]);
        text = "";
    }
    // Esc on the page with a search typed (a result has the focus) clears it and comes back here.
    Component.onCompleted: owner.unsearch = () => {
        if (owner.query === "")
            return false;
        text = "";
        search.forceActiveFocus(Qt.ShortcutFocusReason);
        return true;
    }
    Keys.onDownPressed: owner.found.enter()
    Keys.onEscapePressed: event => {
        if (text === "") {
            event.accepted = false;
            return;
        }
        text = "";
    }
}
