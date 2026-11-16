pragma Singleton

import QtQuick

QtObject {
    function find(item, test) {
        if (test(item))
            return item;
        const kids = item.contentItem ? [...(item.children ?? []), item.contentItem] : (item.children ?? []);
        for (const kid of kids) {
            const found = find(kid, test);
            if (found)
                return found;
        }
        return null;
    }
}
