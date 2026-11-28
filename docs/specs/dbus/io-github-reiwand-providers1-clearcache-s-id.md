# `io.github.reiwand.Providers1`: `ClearCache` to `RemovePlugin`

On this page: `clearcache`, `sync →`, `collections →`, `reseed →`, `setsecret`, `forgetsecret`, `setsecretstore`, `inspectplugin →`, `installplugin →`, `listplugins →`, `removeplugin`.

## `ClearCache(s id)`

Deletes the photos a provider brought: its items leave the library with their thumbnails and the
files of its cache folder are deleted. The provider, its options, its secret, its library source
and `.credits.json` stay, so the next scheduled sync keeps its time and brings new photos. A
running sync of the provider is cancelled first. A photo of the provider that a screen shows is
treated as `Remove` treats it. `NotFound` for an unknown id.

## `Sync(s id) → (t job)`

Starts a `provider_sync` job; `id = ""` syncs every provider that is not `needs_secret`. A
provider that another sync job is already syncing is skipped. `NeedsSecret` when the provider's
required secret is missing.

## `Collections(s id, s owner) → (s json)`

A `CollectionsReply` for a `wallhaven` provider: the collections of the user `owner`, or, when
`owner = ""`, those of the account of the stored API key, private ones included
(`GET /collections`). Their `id` and the owner make the option `collection` (`<owner>/<id>`).
`InvalidArgs` for another kind, an invalid owner, or no owner and no key. Wallhaven answers at
most 45 requests per minute; the daemon spaces its requests to that limit.

## `Reseed(s id) → (t job)`

Starts the sync of a `wallhaven` provider (`Sync` rules). Every sync, scheduled or not, already
draws a new seed for `sorting = random`, so `Reseed` is the same as `Sync` for this kind.
`InvalidArgs` for another kind.

## `SetSecret(s id, s secret)`

Stores the provider's secret, replacing the previous one. `InvalidArgs` for an empty secret or
a kind without secret.

## `ForgetSecret(s id)`

Deletes the provider's secret. Kinds without secret never touch the keyring.

## `SetSecretStore(s store)`

Chooses where this computer keeps every secret of the program: the passwords and API keys of
providers, the secret texts of scene properties and the session token of every account
(`Accounts1`). `store` is `keyring` (the Secret Service, the default) or `file` (`InvalidArgs` for
anything else): a private file `secrets.json` (mode 0600, replaced whole through a temporary file)
in the data folder, for a computer that has no keyring. The choice is the existence of that
file, so there is one place that decides and nothing else to keep in step. Weakening secrecy is the
person's explicit choice: clients show the notice before they call it with `file`, and show a
notice that stays while `List().store` is `file` (whoever can read the files of the person can use the secrets
and read what an encrypted account syncs) and, with the keyring chosen but none found
(`keyring_found = false`), say so with the choice. Switching moves every secret in the direction
asked, with nothing removed or linked again: to the file, it reads all of them from the keyring
(the unlock prompt may show; if one cannot be read nothing changes), writes the file, then deletes
the keyring items; to the keyring, it needs a Secret Service (`KeyringUnavailable`, and the
file stays), stores them all and then deletes the file. Secrets never appear in replies, logs,
errors or signals. `Changed("providers")` and `Changed("account")` follow; `GetStatus` reports it
as `file_secrets`.

## `InspectPlugin(s path) → (s json)`

Reads a provider plugin ([provider-plugins.md](../provider-plugins.md)) from the absolute `path` of
its folder or zip, checks its format, limits and every module (types included), and installs
nothing. Returns a `PluginOffer`: `plugin` (a `PluginInfo`: `id`, `kind` = `plugin:<id>`,
`version`, `title`, `license`, every host it contacts in `hosts`, `secret` = `{label, header,
hosts, required}` or `null`, `sources`, the providers that use an installed plugin with this id,
and `error` `null`), `installed`, the version installed now or `null`, `added_hosts`, the hosts the
installed version does not contact (all of them for a new plugin), `secret_changed`, whether the
secret it asks for differs from the installed version's, and `token`, a digest of every file of
the plugin. Clients show the hosts and the secret and ask the user before `InstallPlugin`; an
update asks again, naming `added_hosts`. `InvalidArgs` for a relative path or a plugin that is
refused, with the reason (the file, field or line); `Failed` for a file that cannot be read.

## `InstallPlugin(s path, s token) → (s kind)`

Installs the plugin at `path` after the user agreed to what `InspectPlugin` showed: `token` is the
one it returned, and the plugin is read again and installed only when its files still give that
token, so exactly the hosts and secret shown are installed. Installing the same id again replaces
it whole; the providers that use it keep their options and secret and use the new version at their
next sync. Returns the provider kind `plugin:<id>` for `Add`. `InvalidArgs` for an empty token
(nothing was shown), a relative path or a refused plugin; `Conflict` when the files changed since
they were shown (`show it again and ask`). `Changed("providers")` follows.

## `ListPlugins() → (s json)`

A `PluginsReply`: `plugins`, one `PluginInfo` per installed plugin, sorted by id. A plugin whose
files cannot be read any more is listed with its id, empty texts and its reason in `error`.

## `RemovePlugin(s id)`

Deletes an installed plugin. `InvalidArgs` while providers use it, naming them (`the provider plugin
<id> is the provider of a, b: remove them first`); `NotFound` for a plugin that is not installed.
`Changed("providers")` follows.
