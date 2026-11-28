# Packages

A `.wpak` package is a ZIP file with `manifest.json` at its root, the entry it names, and the
files that entry uses. `reiwand-format` reads it strictly: stored or deflated entries only, no
encryption, no symbolic links, safe relative paths (no `..`, `\`, `:` or control characters,
at most 255 bytes), names unique without regard to case, at most 4096 files, 1.5 GiB per file and
2 GiB in total, a compression ratio of at most 100 for files above 1 MiB, and a manifest of at
most 1 MiB. Repositories (`repo.md`) and the server (`server.md`) distribute packages; scenes are
described in `scene.md`. An author signs the exact bytes of `manifest.json` into
`manifest.json.minisig`, and its trusted comment names the digest of every other file of the
package (`repo.md`, "Package signatures"): changing any file after `pack sign` voids the signature.

`pack sign` writes the signature with the same trusted comment as always and adds, in its untrusted
comment line (which every verifier ignores), `wpak manifest sha256:<hash of manifest.json>`. Signatures
whose trusted comment does not name the digest of the files, made by earlier versions, no longer verify
and must be made again with `pack sign`. `pack build` never embeds a signature that does not match the
folder: it refuses one whose trusted comment names other files (`it signs other files than this folder
holds`), with the hash it compares the manifest, and it also verifies the signature against the keys given with `--key`; a signature
without the hash and without `--key` is refused with the advice to sign again or pass the key (a key is
never inferred from the folder).

| `kind` | `entry` | Other files |
|--------|---------|-------------|
| `image` | png, jpg, jpeg, webp, avif, gif, bmp or ktx2 | images, txt |
| `video` | webm, mp4 or mkv | images, videos, txt |
| `scene` | `scene.json` | images, videos, json, wgsl, js, gltf, glb, bin, ogg, opus, flac, mp3, wav, ttf, otf, ttc, otc, woff2, cube, txt |
| `web` | an html or htm page, usually `index.html` | images (also svg, hdr and exr), videos, html, htm, js, mjs, css, json, wasm, ogg, opus, flac, mp3, wav, ttf, otf, woff, woff2, glb, gltf, bin, obj, mtl, fbx, basis, glsl, vert, frag, wgsl, csv, xml, txt |
| `preset` | `preset.json` | images, videos, json, txt |
| `assets` | a scene document such as `assets.json` | as `scene` |

Every kind may also carry `locale/<language>.json` (see "Languages"). No kind admits programs or
libraries (exe, dll, so, sh, bat, AppImage and the like are refused by their extension), and a
`.wasm` file must start with the WebAssembly signature.

The specification, part by part:

- [Versions and Manifest](package/versions.md): versions, manifest.
- [Display and playback and Rating and photosensitivity](package/display-and-playback.md): display and playback, rating and photosensitivity.
- [Inputs](package/inputs.md): inputs.
- [Features](package/features.md): features.
- [Profiles, Platforms, Dependencies, Origin and Web pages](package/profiles.md): profiles, platforms, dependencies, origin, web pages.
- [Presets, Shared resources, Languages and Markup](package/presets.md): presets, shared resources, languages, markup.
- [Facets and tags](package/facets-and-tags.md): facets and tags.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="versions"></a>[Versions](package/versions.md#versions)
- <a id="manifest"></a>[Manifest](package/versions.md#manifest)
- <a id="display-and-playback"></a>[Display and playback](package/display-and-playback.md#display-and-playback)
- <a id="video-settings"></a>[Video settings](package/display-and-playback.md#video-settings)
- <a id="rating-and-photosensitivity"></a>[Rating and photosensitivity](package/display-and-playback.md#rating-and-photosensitivity)
- <a id="inputs"></a>[Inputs](package/inputs.md#inputs)
- <a id="rgb-lighting"></a>[RGB lighting](package/inputs.md#rgb-lighting)
- <a id="features"></a>[Features](package/features.md#features)
- <a id="profiles"></a>[Profiles](package/profiles.md#profiles)
- <a id="platforms"></a>[Platforms](package/profiles.md#platforms)
- <a id="dependencies"></a>[Dependencies](package/profiles.md#dependencies)
- <a id="origin"></a>[Origin](package/profiles.md#origin)
- <a id="web-pages"></a>[Web pages](package/profiles.md#web-pages)
- <a id="presets"></a>[Presets](package/presets.md#presets)
- <a id="shared-resources"></a>[Shared resources](package/presets.md#shared-resources)
- <a id="languages"></a>[Languages](package/presets.md#languages)
- <a id="markup"></a>[Markup](package/presets.md#markup)
- <a id="facets-and-tags"></a>[Facets and tags](package/facets-and-tags.md#facets-and-tags)
