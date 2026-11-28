# Providers

Providers are online sources of wallpapers. This page covers the Wallhaven provider and provider
plugins, which add sources Reiwand does not have built in.

## Wallhaven

The Wallhaven provider uses the official v1 API and supports all the options of its search:

| Option | Values |
|--------|--------|
| words | `query` |
| categories and purity | |
| order | `date_added`, `relevance`, `random`, `views`, `favorites`, `toplist`, `hot` |
| direction | |
| `toplist` range | `1d` to `1y` |
| minimum resolution and exact resolutions | |
| ratios | `16x9`, `16x10`, `landscape`, `portrait`… |
| colors | of its palette |
| "similar to" a wallpaper | `similar` |
| a tag | `tag` |
| a user | `uploader` |
| collections | `collection = user/id`, public or, with your API key, private |
| starting page | |
| pages read per sync | 1 to 10 |
| maximum of images | up to 240 |

Each option is:

- an option of `reiwand-ctl provider wallhaven add|set` (`--help` explains them; `set` changes
  several at once and checks the set);
- a field of `Providers1`;
- a control of the "Search options" dialog of the app: checkboxes for categories and purity,
  buttons for ratios and resolutions, a color palette and, for collections, a picker that lists them
  with `reiwand-ctl provider wallhaven collections` (with an API key it includes yours).

Example:

```sh
reiwand-ctl provider wallhaven add art --ratios 16x9,landscape --colors 660000 --sorting random --no-secret
```

### Random order

With `random` order, the daemon chooses a new seed on each sync and uses it on all its pages so they
do not repeat images. `reiwand-ctl provider wallhaven reseed` (or "New random order" in the app)
syncs at once.

### Pasting a search address

A search address from wallhaven.cc can also be pasted in the query. Its parameters become the
options.

### Limits and API key

- A sync asks only for the configured pages (one by default).
- It never goes over the API's 45 requests per minute, and waits and retries when it answers 429.
- `sketchy` and `nsfw` stay off by default and ask for the API key.
- The API key only goes in the `X-API-Key` header and is stored in the keyring.

On the phone there are no provider settings.

## Provider plugins

A provider plugin adds a source Reiwand does not have built in: a folder or zip of readable
TypeScript ([how to write one](../../authoring/provider-plugins.md)).

1. Run `reiwand-ctl provider plugin install PATH`. It shows its title, version and licence, every
   host it contacts and the secret it asks for.
2. Answer yes to install it (`--yes` for scripts that showed it already). Installing a new version
   shows it again and names the hosts it adds.
3. Run `reiwand-ctl provider add ID plugin:ID KEY=VALUE...` to make a source of it. The plugin's
   settings are its options, and its secret is asked for as for any provider.

```sh
reiwand-ctl provider plugin install gallery.zip    # a provider plugin: shows its hosts and secret and asks; then provider add ID plugin:ID
```

The plugin runs in a sandbox that reaches only its hosts, over https and public addresses.

| Command | What it does |
|---------|--------------|
| `reiwand-ctl provider plugin ls` | lists the plugins and the providers that use them |
| `provider plugin rm ID` | removes one that no provider uses |
