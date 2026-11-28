import QtQuick
import io.github.reiwand.app

// The panels of the package as a wallpaper: its sheet, its texts, how it will look and the review
// before publishing; each made the first time it shows, then kept in the group holding it.
Item {
    id: sheets

    required property var area

    SceneLazyUnit {
        area: sheets.area
        unit: "sheet"

        SceneSheet {
            anchors.fill: parent
            page: sheets.area.page
        }
    }

    SceneLazyUnit {
        area: sheets.area
        unit: "texts"

        SceneTextsPanel {
            anchors.fill: parent
            page: sheets.area.page
        }
    }

    SceneLazyUnit {
        area: sheets.area
        unit: "cover"

        SceneCoverPanel {
            anchors.fill: parent
            page: sheets.area.page
        }
    }

    SceneLazyUnit {
        area: sheets.area
        unit: "review"

        SceneReviewPanel {
            anchors.fill: parent
            page: sheets.area.page
        }
    }
}
