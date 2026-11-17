import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property var adder: page.bar.adder
    readonly property var error: "io.github.reiwand.Error."

    function expect(ok, what) {
        if (!ok)
            throw new Error("repository dialog: " + what);
    }

    function fail(name, message) {
        adder.told(error + name, message);
    }

    function exercise() {
        expect(page.bar.link.visible && page.bar.link.text === App.tr("Add a repository"), "the add button must always be visible");
        adder.offer("https://casa.example/fondos");
        expect(adder.address.text === "https://casa.example/fondos" && adder.stage === "ask", "the address must arrive in the field");
        expect(adder.address.width <= adder.width - 2 * Look.e4, "the field must fit inside the dialog");
        adder.address.accepted();
        expect(adder.stage === "working", "Enter in the field must start the add");
        fail("CertificateUnknown", "the certificate of casa.example is not from a known authority; its fingerprint is abcd12ef; add it again");
        expect(adder.stage === "certificate" && adder.reply.digest === "abcd12ef" && adder.reply.trust.visible, "an unknown certificate must offer to trust it");
        adder.reply.trusted();
        expect(adder.stage === "working", "trusting must add again");
        fail("NotARepository", "this is a server, not a repository: paste the address of one of its repositories: Home (https://casa.example/fondos), Art (https://casa.example/arte)");
        expect(adder.stage === "server" && adder.reply.found.length === 2 && adder.reply.servers.visible, "a server address must list its repositories");
        adder.reply.servers.incrementCurrentIndex();
        adder.reply.servers.pick();
        expect(adder.address.text === "https://casa.example/arte" && adder.stage === "working", "choosing a listed repository must add it");
        fail("NotARepository", "this is a server, not a repository: Casa has no public repository");
        expect(adder.stage === "server" && adder.reply.found.length === 0 && !adder.reply.servers.visible, "a server with nothing listed shows only the reason");
        adder.address.text = "https://repo.example.org/r";
        adder.listed([{
                "id": "a1",
                "server": "https://repo.example.org",
                "user": "reader",
                "pending": false
            }, {
                "id": "a2",
                "server": "https://other.example",
                "user": "other",
                "pending": false
            }]);
        expect(adder.stage === "account" && adder.reply.choices.entries.length === 1 && adder.reply.choices.entries[0].label === App.tr("Use the account %1", "reader"), "only the accounts of that server must be offered");
        expect(adder.reply.another.text === App.tr("Use another account"), "another account must be offered");
        adder.use("a1");
        expect(adder.stage === "working", "using an account must add again");
        adder.listed([]);
        expect(adder.reply.another.text === App.tr("Sign in") && !adder.reply.choices.visible, "with no account the dialog asks to sign in");
        adder.approving(["ABCD-EFGH", "https://repo.example.org/link", 21]);
        expect(adder.stage === "approve" && adder.reply.code === "ABCD-EFGH" && adder.reply.open.visible, "the sign-in must show its code and its link");
        adder.ended(21, "done", "", "");
        expect(adder.stage === "working" && adder.kind === "add", "an approved sign-in must add again");
        adder.approving(["ABCD-EFGH", "https://repo.example.org/link", 22]);
        adder.ended(22, "failed", error + "Failed", "denied");
        expect(adder.stage === "failed" && adder.reply.detail === "denied", "a refused sign-in must say so");
        fail("CertificateChanged", "the certificate of casa.example is not the one you trusted");
        expect(adder.stage === "warning", "a changed certificate must warn");
        fail("SignatureChanged", "the signature changed");
        expect(adder.stage === "warning", "a changed signature must warn");
        adder.address.text = "https://casa.example/other";
        adder.address.textEdited();
        expect(adder.stage === "ask", "editing the address must start over");
        adder.submit();
        adder.started(30);
        adder.ended(30, "done", "", "");
        expect(adder.address.text === "", "an added repository must clear the field");
        adder.close();
        expect(page.bar.off.visible === App.cut, "the way out of private mode must show only in private mode");
    }
}
