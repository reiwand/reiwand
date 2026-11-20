import QtQuick
import QtTest
import "plasmoid/contents/ui"

Item {
    width: 400
    height: 300

    Consent {
        id: consent

        daemon: QtObject {
            function tr(text, first) {
                return [text, first].filter(part => part !== undefined).join(" ");
            }

            function act() {
            }
        }
    }

    TestCase {
        function scene(name) {
            return {
                "scene": name,
                "key": "sha256:" + name,
                "title": name,
                "pointer": true,
                "audio": true,
                "granted": []
            };
        }

        function test_a_preview_never_asks() {
            consent.inputs = {
                "shown": {
                    "preview:card-1794": scene("sea"),
                    "preview:editor": scene("pond"),
                    "preview:page-lights": scene("rain")
                }
            };
            compare(consent.ask.scene, "");
            compare(consent.ask.inputs.length, 0);
            verify(!consent.visible);
        }

        function test_a_screen_asks_beside_a_preview() {
            consent.inputs = {
                "shown": {
                    "preview:card-1": scene("sea"),
                    "DP-1": scene("pond")
                }
            };
            compare(consent.ask.screen, "DP-1");
            compare(consent.ask.scene, "pond");
            compare(consent.ask.inputs.join(), "audio,pointer");
            consent.output = "preview:card-1";
            compare(consent.ask.screen, "preview:card-1");
            consent.output = "";
        }

        name: "tray-consent"
    }
}
