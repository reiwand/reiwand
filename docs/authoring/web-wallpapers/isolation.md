# What a page may reach

On this page: chosen folders, what the page sees, the isolation and the network.

## Chosen folders

A `folder` property lets the user choose a folder of photos or videos. The page never sees paths,
but `wpak-user://` addresses that only work inside it:

```js
const one = await reiwand.randomFile("photos");
image.src = one;
```

With `"mode": "all"` you also have the whole list and the changes:

```js
const all = await reiwand.files("photos");
reiwand.onFiles("photos", ({ added, removed, changed }) => update(added, removed, changed));
```

`changed` brings `[before, now]` pairs of the files that were rewritten with the same name: the new
address loads the new content. The daemon lists the folder (`max_files`, `recursive`; without
hidden files or links) and looks at it again every 5 s while the page is in view.

## What your page sees

By default it sees the time zone (`new Date()` gives local time), the language
(`navigator.language`), what the graphics card can do through WebGL (on the desktop and on the
phone, also the model of the card) and its own storage. The user can hide each thing: then the page
sees UTC, `en-US` (also in `Intl` and `toLocaleString`), a generic card name (`WebKit`, `WebKit
WebGL`; WebGL keeps working; `navigator.gpu` disappears; workers cannot start) and nothing stored,
in the page, in each of its frames and in its workers. Make it work in all those cases. It never sees paths, device names,
the battery or the list of fonts.

## What the isolation forbids

Each layer applies to every page:

1. It only reads its own package, through `wpak://<package id>/<path>` addresses, and never
   `manifest.json` or its signature. Files the user chooses arrive only through `wpak-user://`.
2. A content policy that the page cannot remove limits loads to `wpak:`, `wpak-user:`, `data:` and
   `blob:` (plus the declared origins when the network is allowed); there are no objects, forms or
   `<base>`.
3. Everything else is blocked, `fetch` and WebSockets included.
4. There is no navigation outside the package, new windows, downloads, dialogs, file picker,
   notifications or full screen. Every permission (camera, microphone, location, clipboard…) is
   denied. WebRTC and media capture are off, also in every frame of the page, from the moment it
   is inserted.
5. Without a declared and allowed network, the helper program starts in an empty network space: not
   even the engine's network process reaches anything.
6. The page runs in its own process, inside an isolated environment.

## Network

If you really need it, declare the origins (up to 8, `https://` without a path, in lowercase) and
`network` in `inputs`:

<!-- docs-examples: examples/web-clock/manifest.json -->
```json
"inputs": ["network"],
"permissions": { "network": { "allow": ["https://api.example.org"] }, "rgb": false }
```

The page only reaches those origins and only after the user allows it (`reiwand-ctl input grant
<id> network`, or from the app); meanwhile `fetch` fails. A page that lists origins declares
`network`, and only then. Avoid it if you can: a wallpaper without network works everywhere and
exposes nothing.
