# Provider plugins

A provider plugin adds a source of wallpapers that Reiwand does not have built in: a self-hosted
gallery, an art site with an API. It is written in TypeScript and shipped as readable source, never
as a binary. The built-in providers (`crates/reiwand-providers`) are unchanged. The engine is the
crate `crates/reiwand-plugin`; it is shared by the desktop and the phone.

## Format

A plugin is a folder, or a zip of that folder (one top folder inside the zip is allowed):

```
example-gallery/
  reiwand-provider.json   what the plugin is, the hosts it contacts, its secret and settings
  main.ts                 exports list()
  lib/*.ts                more modules, imported by relative path
  LICENSE, README.md      optional texts
```

- Only `reiwand-provider.json`, `.ts` modules (at most 4 folder levels), `LICENSE*` and `README.md`
  are accepted; anything else (a binary, a hidden file, a link) refuses the plugin and names the file.
- Limits: at most 64 files, 256 KiB per file and 1 MiB in all.

`reiwand-provider.json` (schema `reiwand.provider/1`) stores every value, as every Reiwand format
does ([explicit-values.md](explicit-values.md)): a missing field refuses the file and names its path
(`reiwand-provider.json: "settings[1].max" is missing`).

| Field | Value |
|---|---|
| `schema` | `"reiwand.provider/1"`; any other value is refused as another version |
| `id` | a lowercase letter, then at most 31 lowercase letters, digits, `-` or `_` |
| `version` | a semantic version |
| `title` | at most 80 characters |
| `license` | the plugin's licence, such as `GPL-3.0-or-later` |
| `hosts` | 1 to 16 exact host names (lowercase, no scheme, port, wildcard or IP address) |
| `secret` | `null`, or `{label, header, prefix, hosts, required}` (written as `null` when absent) |
| `settings` | up to 32 fields `{key, label, kind, prefill, min, max, values}` |

- `secret.header` is the HTTP header that carries the secret (`Host`, `Cookie` and the framing
  headers are refused); `prefix` is written before it (`"Bearer "`, or `""`); `hosts` are the
  declared hosts that receive it (a subset of `hosts`); `required` says whether the plugin works
  without it.
- A setting's `kind` is `text`, `int`, `bool` or `choice`. `min` and `max` are numbers only for an
  `int` (or `null` for no bound) and `null` for the others; `values` lists the words of a `choice`
  and is `[]` for the others. `prefill` is the value the form offers when a source is added; the
  saved source stores every value. Keys Reiwand uses itself (`kind`, `refresh`, `cache_mb`,
  `keep`, `password`, `api_key`, `token`, `secret`, `allow_plain_http`) are refused.
- Prefill table: `crates/reiwand-plugin/src/prefill.rs` (`secret`, `min` and `max` are values:
  their `null` has a meaning). Tests: `crates/reiwand-plugin/tests/format.rs`.

## Install

1. `Providers1.InspectPlugin` (`reiwand-ctl provider plugin install PATH`) reads and checks the
   plugin: format, limits and every module, types included. Nothing is installed.
2. The user is shown the title, version, licence, every host the plugin contacts and the secret it
   asks for (label and the hosts that receive it), and installs only after saying yes. An update
   shows the same and names the hosts the installed version did not contact, and asks again.
3. `Providers1.InstallPlugin` takes the token of what was shown (a digest of every file) and
   installs only if the files still give it: a plugin whose hosts or code changed in between is
   refused (`Conflict`) and must be shown again.
4. The TypeScript types are removed (`swc_ts_fast_strip` in strip-only mode, which replaces types
   with spaces so lines and columns stay the same). Syntax that needs more than removing types is
   refused with its file, line and column: `enum`, `namespace` with values, parameter properties,
   `import x = require()`, `export =`, `<T>value` assertions and decorators.
5. The plugin is written to `<plugins>/<id>/` (`~/.local/share/reiwand/plugins/` on the desktop):
   `source/` holds the files as they were shipped and `js/` the modules without types. Installing
   the same id again replaces it whole.

`Providers1.ListPlugins` lists the installed plugins with the providers that use each one, and
`RemovePlugin` deletes one that no provider uses (otherwise it names them).

## As the provider of a source

An installed plugin is the provider kind `plugin:<id>` (`reiwand-ctl provider add ID plugin:<id>
KEY=VALUE...`). Its options are the declared settings, completed with their `prefill`, plus
`refresh` (one day by default) and `cache_mb`, all stored in `config.toml` like those of the
built-in kinds. The secret is given when the provider is added (or with `SetSecret`) and stored in
the same keyring (or private file) as the secrets of the built-in providers, under the provider
id; a required one missing leaves the provider in `needs_secret`.

A sync calls `list` for at most 5 pages and 500 items (repeated ids count once), then downloads
each item's file through the cache of the provider, with the same limits as the built-in kinds
(`cache_mb`, protected files, eviction). The download uses the guard of `fetch`: https only, the
declared hosts on the standard port, names with only private, loopback or reserved addresses
refused, no system proxy, the private mode, and redirects followed (up to 5) only to declared hosts,
and only to `secret.hosts` when the request carries the secret. The secret goes with the file
request only to `secret.hosts`. The item's author and licence are its credit.

## Running

`main.ts` exports `async list({settings, page})`:

- `settings` holds every declared setting, typed (`int` as a number, `bool` as a boolean).
- `page` is `null` for the first page, then the `next` cursor of the page before.
- It returns `{items, next}`. `next` is a cursor text or `null` at the end. Each item stores every
  field: `{id, title, media, url, page, author, license}`; `media` is `"image"` or `"video"`; `url`
  is the file, on a declared host over https; `page` is the item's web page (https) or `null`;
  `author` and `license` are texts or `null`. At most 500 items per page. A page that breaks a rule
  is refused naming the item and field (`"items[3].url" is not an https address on a host the
  plugin declares`).

The host gives the script one global beyond what scene scripts get, `fetch(url, {method, headers,
body})`:

- Only `GET` and `POST`, only `https`, only the declared hosts on the standard port, no user name
  in the address. Redirects are followed by the host, up to 5, only to declared hosts.
- Names that resolve only to private, loopback, link-local or reserved addresses are refused; the
  private mode of the network (`reiwand-ctl private on`) cuts plugins too. Plugin requests do not
  go through a system HTTP proxy: the proxy would resolve the names itself and the address check
  would not hold.
- The answer has `status`, `ok`, `url`, `headers.get(name)`, `text()` and `json()`. A body is at
  most 4 MiB.
- The secret is attached by the host as the declared header, only to `secret.hosts`, also on each
  redirect hop. The script never receives it and may not set that header itself. A server that
  echoes request headers in its body would show it to the script: only declare hosts you trust with
  the secret.
- The script has no files, processes, module loading from outside the plugin, or other network. Its
  modules import each other by relative path ending in `.ts`.

Limits of each call to `list`: 32 MiB of memory, 2 s of processor time (an interrupt handler stops
loops), 60 s in all, 32 requests. A refused request, a limit or a broken page stops the call even if
the script catches the error; a request that fails on the network (timeout, no answer) is thrown to
the script, which may handle it.

On the desktop each call runs in its own process (`--provider-plugin-host`) under the seccomp
sandbox of the scene scripts (`reiwand_script::sandbox`): that process has no files and no network;
it asks the service for each `fetch` over its pipe. On the phone the call runs in the app's process
with the same limits.

## Errors

`reiwand_plugin::Error` tells what stopped the plugin, in words: a format fault with its path, a
type-stripping fault with file, line and column, a missing or wrong setting, the processor time,
total time or memory limit, a refused request (with the reason), a limit, a script error, a page
that is not valid, or a fault of the plugin host.

## On the phone

The phone's native library links this crate. Requests to `account` (`android/app`, JSON with an
`op`): `plugins` lists the installed ones (as `ListPlugins`); `plugin_inspect` with `path` (a
folder or zip; the app copies a picked zip into its cache) answers what the consent shows, with
`added_hosts`, `secret_changed`, `installed` and `token`; `plugin_install` with `path` and that
`token` installs it, refused when the files changed; `plugin_remove` with `id`, refused while a
source uses it. Installed plugins add kinds `plugin:<id>` to `providers`, with each setting's
`label` and type, and `refresh` and `keep` as every phone source has. Plugins live in
`plugins/` of the app's files, in the same layout as on the desktop.
