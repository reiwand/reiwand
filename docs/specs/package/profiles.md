# Profiles, Platforms, Dependencies, Origin and Web pages

On this page: profiles, platforms, dependencies, origin, web pages.

## Profiles

Each platform is a profile: a column of the tables above that says, for every feature, whether that
platform runs it, plays the package without it or refuses the package. The registry in
`crates/reiwand-format` (`features::all()`, `features::support`) holds both columns, and every reader
uses it: the desktop daemon and library, `reiwand-ctl`, the app, repositories and the server (which
compute facets with it) and the Android app. A package is refused on a platform when any of
its features is refused there, with every such feature and its reason in one message; the features
a platform leaves out are listed on the package's page there; and the facets `desktop` and
`android` name the platforms that run a package whole. The package limits of the first part of this
page are the same on both platforms. The tests build one small package per feature
(`tests/fixtures/features.json`) and run it through `pack build`, the desktop daemon and the Android
library, so that a feature without its package, or a package that reaches an engine that cannot
draw it, fails them.

## Platforms

`platforms` lists the devices a package is made for, `desktop`, `android` or both, and can only
narrow where it plays: a package made for the phone only is refused on the desktop with `this
package is made for android only`, and the reverse. Facets name each platform that runs the
package fully (no feature refused or left out) and that its `platforms` allows, so that `facet:android`
finds what plays whole on the phone. When they record facets (adding to a repository, indexing the
library, the server and `pack check`), readers also check the limits of the phone: a package that
takes more than 1 GiB once unpacked, or holds a png, jpeg, webp or gif picture of more than 8192
pixels a side or 32 megapixels, loses the `android` facet and `pack check` says why. `pack check` of a folder does the same check on its files, and the editor lists it among the problems of the folder as a warning on `/manifest`, so that the author sees it before building. The phone opens
packages with that 1 GiB limit, so it refuses larger ones. The app shows where a package works fully
in its details and `reiwand-ctl lib show` prints it as `works on`. Repositories copy `platforms` into the
release.

## Dependencies

`requires.packages` lists at most 16 other packages, each `{id, version}` with a SemVer requirement
such as `^1.2`, that a package needs installed; it cannot name the package itself or one id twice.
`requires.gpu` is `any` or `compute` (`null` and `any` mean the same: no compute shaders needed) for packages that cannot draw without compute
shaders; they get the `compute` facet, and where the graphics card lacks compute shaders or draws in
software (`compute` of `GetState`) the app, `reiwand-ctl lib apply` and the page of the package on the
phone warn before it is applied, which it still is. Installing a package from a repository also
installs, from the same repository, each package of `requires.packages` that no installed version
matches. Repositories copy `requires` into the release.

## Origin

An `imported` package is a conversion for personal use: it installs and plays locally, but
`reiwand-ctl repo add` and the server never publish it. A `provider` package names where its picture
or video comes from: `name` (`[a-z0-9-]`, at most 32), `item_url` and `terms_url` (`https://`),
and the `credit` that is always shown with it; it is published only under `CC0-1.0`, `CC-PDM-1.0`,
`CC-PDDC`, `CC-BY-2.0`, `CC-BY-3.0` or `CC-BY-4.0`, alone or in an `OR` of them. `source_url` is
the `https://` page of the original work. Clients show the origin, the provider's credit and the
packages that credited files come from with the details of the package.

## Web pages

A `web` package shows its entry page in an isolated web view, without network unless allowed
(`web.md` describes the engine, the sandbox, what the page may see and its interface):

```json
{ "kind": "web", "entry": "index.html", "inputs": ["audio", "pointer", "network"],
  "web": { "preload": "compat.js", "background": "#000000", "storage_kib": 512 },
  "permissions": { "network": { "allow": ["https://api.example.org"] } },
  "playback": { "max_fps": "display", "volume": 0.5 },
  "requires": { "engine": null, "features": ["inputs.audio", "inputs.network", "inputs.pointer", "playback2", "web"], "packages": [], "gpu": null } }
```

`web.preload` is a `.js` or `.mjs` file of the package run at the start of the page, before its own
scripts; `web.background` (`#rrggbb`) shows while the page loads; `web.storage_kib` (0 to 5120; 0 prefilled) is the local storage the page may keep: none without it, and what it keeps stays on
the device, is never synced and goes away with the package or a *Clear data*. `permissions.network.allow`
lists at most 8 `https://` origins (`https://host` or `https://host:port`, lowercase, without a
path), the only places the page may reach once the user allows the network; a page that lists any
declares `network` in `inputs`, and only then. Pages receive their settings through `window.reiwand`:

| Member | What the page gets |
|--------|--------------------|
| `properties`, `onProperties(cb)` | the values of every property when it loads, then only the changed ones; colors as `#rrggbb[aa]` and `{hex, r, g, b, a}` with `r`, `g`, `b` and `a` from 0 to 1, combos with their value and `label` |
| `fps`, `quality`, `locale`, `volume`, `onSettings(cb)` | the frame rate it may draw at, and `{fps, quality, locale, volume}` when they change: the quality level (0 to 3), the language picked among those of the package and the wallpaper volume (0 to 1) for its own sound |
| `onPause(cb)` | `true` while it is hidden or paused, when animation frames stop too |
| `onAudio(cb)` | 64 bands per channel while `audio` is allowed, silence otherwise |
| pointer and clicks | real DOM events while `pointer` is allowed, and clicks only from real clicks on screens where clicks are on, for packages allowed `clicks` |
| `onMedia(cb)` | what the media player plays while `media` is allowed (and, for a page that declares network origins, a second confirmation that the title, artist and cover may then leave the computer), with the cover as a `blob:` address and its colors; `{enabled: false}` otherwise (`web.md`) |
| `files(prop)`, `randomFile(prop)`, `onFiles(prop, cb)` | opaque addresses (`wpak-user://<prop>/…` on the desktop, `https://<host>/:user/<prop>/…` on the phone) for the files of a chosen folder, and the ones added and removed; never their paths |
| `onAction(cb)` | the id of a pressed `button` property |
| `openShortcut(prop)` | opens what the user chose for a `shortcut` property, only inside a real click and at most once a second |
| `rgb.setImage(bytes, w, h)` | sends a picture of `w` × `h` RGBA bytes (up to 4096 a side, such as the `data` of `getImageData`) to the RGB lighting of the computer, shrunk on the page to at most 64 × 16, at most 30 times a second; it returns `true` only when the colors go to the lights: with `permissions.rgb`, once the user allows it for the package and while RGB lighting is on (and in the previews of the app and the editor, which show the strip instead of sending it); without them, when the call came too soon or the size does not match it returns `false` and sends nothing. On the phone, which has no RGB lighting, it always returns `false` |

Nothing reaches the page without the input it needs, and a page never sees paths, device names,
the battery or the list of fonts. It sees the time zone, the language and what the graphics card
can do through WebGL, and keeps its storage, until the user restricts each of them for the package; every client
shows a notice saying so (`web.md`, "Privacy"). `onMedia` gets `{enabled: false}` on engines that do
not read the media player. Web pages with storage get the `storage` facet.

On the phone, without the `network` input allowed a page reaches no address at all, and peer
connections (WebRTC) are closed for every page, allowed or not, in all of its frames (`web.md`,
"On the phone").
