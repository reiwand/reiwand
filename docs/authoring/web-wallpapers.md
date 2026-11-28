# Web wallpapers

A web wallpaper is a page that Reiwand shows behind the windows. Everything it uses goes inside the
package: there is no server, the network is cut off unless you declare it and the user allows it,
and the page cannot open windows, dialogs or downloads. This guide explains how to make one;
`docs/specs/package.md` ("Web pages") describes the manifest and `docs/specs/web.md` the engine and
the isolation. `docs/authoring/examples/web-clock` is a complete, minimal web wallpaper; in the
source tree `scenes/lights` is a larger one, with RGB lights (it does not ship with the installed
program).

Web wallpapers, by topic:

- [The minimum web wallpaper](web-wallpapers/basics.md): the smallest page and where it shows.
- [The page and the program](web-wallpapers/interface.md): `window.reiwand`, testing and debugging.
- [What a page may reach](web-wallpapers/isolation.md): chosen folders, what the page sees, the isolation and the network.
- [Phones and existing applications](web-wallpapers/phone-and-apps.md): web wallpapers on the phone and turning an application into one.
