# Conventions, Errors and Streams and claims

On this page: conventions, errors, streams and claims.

## Conventions

- Members are PascalCase; argument names are snake_case and are part of the interface.
- Queries return one `s`: a JSON object whose first fields are `model` (the data model version,
  property `Model`) and `serial` (the value of property `Serial` read before the query ran). The
  shapes are listed in "JSON model".
- Mutations take typed arguments and return nothing or a created identifier. Everything is
  validated before anything changes: a failed call changes nothing. Calls that take several
  uids check every uid first and change them in one transaction.
- Operations that walk folders, copy files or touch the network return a job id at once and run
  in the background (see "Jobs").
- Library identifiers (`uid`) are ULIDs: 26 characters, uppercase Crockford base32. Only full
  uids travel on the bus, except in `Library1.Resolve` and where an argument is documented as
  "uid or name".
- Output arguments are connector names such as `DP-1`, trimmed; more than 128 bytes or control
  characters return `InvalidArgs`. `""` means every output. In the interfaces other than
  `Daemon1`, a name starting with `@` returns `InvalidArgs`.
- Times are `i64` Unix seconds, UTC.
- Paths are absolute and UTF-8. `SetWallpaper` also accepts `file://` URIs; every other method
  rejects URIs with `InvalidArgs`, so clients convert `file://` URIs to paths (percent-decoded)
  before calling.
- Batch arguments (`as uids`, `as paths`, `au indexes`) take at most 10 000 entries (1 000 for
  `Import`, 500 for `RequestThumbnails` and `RequestPreviews`); page `limit` is at most 500.
- Titles have 1 to 256 characters, playlist, profile and preset names 1 to 128, tags 1 to 64;
  all are trimmed and may not contain control characters.

`REIWAND_PRIVATE_BUS_ONLY=1` in the environment of `reiwand-ctl` (or of any client built on the shared
client library) makes it refuse to connect unless `DBUS_SESSION_BUS_ADDRESS` equals
`REIWAND_PRIVATE_BUS_ADDRESS`, which only a run that started its own private bus exports (the
integration scripts do). Set it for any manual or automated run that must never reach the daemon
of the user's own session.

## Errors

Every error carries a readable English message. An error raised without a description carries
`failed without giving a reason (<Name>)`, with the last part of its error name.

| Error name | When |
|------------|------|
| `org.freedesktop.DBus.Error.InvalidArgs` | Malformed or out-of-range argument, bad query syntax, operation not allowed on that object |
| `org.freedesktop.DBus.Error.Failed` | Anything else: I/O, database, verification failure, library unavailable, renderer stopped |
| `org.freedesktop.DBus.Error.NotSupported` | The method is not available in this version of reiwandd |
| `io.github.reiwand.Error.NotFound` | Unknown uid, name, output assignment, provider, repository, package or job |
| `io.github.reiwand.Error.Exists` | Name, folder, provider id or repository already present |
| `io.github.reiwand.Error.NeedsSecret` | A provider needs a secret that is missing or was rejected |
| `io.github.reiwand.Error.SignedOut` | The server refused the token of this computer (`unauthorized`): the device was revoked or the account deleted |
| `io.github.reiwand.Error.AccountNeeded` | The repository requires an account (`401 account_needed`) and none is linked or the linked one is not accepted: the message lists the accounts known on that server; use `Accounts1.Use` or `SignIn` |
| `io.github.reiwand.Error.KeyringUnavailable` | The keyring item cannot be written or deleted, the keyring stayed locked for 20 s or its unlock prompt was dismissed, the Secret Service did not answer in time, or no Secret Service is available to store a secret |
| `io.github.reiwand.Error.Stale` | A repository's signed index expired; retry with `allow_stale = true` after asking the user |
| `io.github.reiwand.Error.Conflict` | A playlist edit by index found other entries than expected: `the playlist changed; reload it` |
| `io.github.reiwand.Error.Newer` | A package needs a newer Reiwand: a newer manifest version, a `requires.engine` this version does not meet or a feature of `requires.features` it does not know (see `package.md`) |
| `io.github.reiwand.Error.FlashingDisabled` | `Apply` of a package that flashes while `playback.allow_flashing` is off (`this wallpaper flashes and flashing wallpapers are disabled; allow flashing in the settings to play it`); turn the setting on and apply again |
| `io.github.reiwand.Error.Offline` | A provider sync, adding or refreshing a repository, or a package download could not reach its server: the connection failed or timed out (`no connection: …` for providers; `cannot reach HOST (what the network answered): there is no network or the server does not answer; …` for adding and refreshing a repository, ending with what was kept) |
| `io.github.reiwand.Error.CertificateUnknown` | The server of a repository has a certificate from no known authority; the message carries its fingerprint, to confirm with `Repos1.AddTrusting` |
| `io.github.reiwand.Error.CertificateChanged` | The server of a repository presents another certificate than the one pinned or confirmed; nothing is applied |
| `io.github.reiwand.Error.SignatureChanged` | A refresh found another `repo_id` at the repository address, or a root that does not chain from the stored one; nothing is applied. `List` adds the warning `SignatureChanged` |
| `io.github.reiwand.Error.NotARepository` | The address is a server, not a repository; the message lists the addresses of its public repositories (from discovery `repositories`) |
| `io.github.reiwand.Error.Untrusted` | A repository index or a package failed its check: a signature or key refused, a digest or size that does not match, an index older than the one already seen or that disagrees with it. A security sign, never a network failure |
| `io.github.reiwand.Error.Unconfirmed` | `Grant`/`GrantPackage` from a caller that must be confirmed: the user is asked in a desktop notification; treat it as pending, not as a failure |
| `org.freedesktop.DBus.Error.AccessDenied` | `Grant`/`GrantPackage` while the same question rests after the user answered *Not now*, closed the notification or left it unanswered (ten minutes, listed in `GetInputs().quiet`), or with no notification service to ask; `SetPointer` from a caller that may not report the pointer |

If `library.db` cannot be opened (for example because a newer build wrote it), the daemon still
renders, `GetState().library_error` holds the reason and every `Library1`, `Playlists1` and
`Profiles1` method, `Providers1.Add`, `Remove` and `Sync`, and `Repos1.Install`, `Uninstall`
and `Remove(uninstall = true)` return `Failed` with it; scheduled syncs are skipped. The other
`Providers1` and `Repos1` methods keep working. An output whose playlist cannot be resolved then
shows its background color and `GetState().outputs[x].error` says why.

A picture, video or package whose file is gone or can no longer be read when its output is drawn
again (deleted, or on a disk that went away, for example when a screen comes back) leaves that
output on its background color, and `GetState().outputs[x].error` says why until another wallpaper
is shown there.

## Streams and claims

A stream exists while at least one claim holds it. Claims are kept per output and per client:

- A desktop client is identified by its unique bus name (the sender of the call). Each
  `RequestStream` adds one claim for that client, so one connection that shows the same output
  several times (for example, several containments) holds several claims. Each `ReleaseStream`
  removes one of them.
- The held claim belongs to no connection. `HoldStream` creates it and it survives the exit of
  the caller; there is at most one per output. `ReleaseStream` from a client without a claim on
  that output removes the held claim.
- When a client leaves the bus (`NameOwnerChanged` with an empty new owner), all its claims are
  removed, even if it never called `ReleaseStream`, and the outputs that use the `span` fit are placed
  again without the streams that went away.
- The stream is destroyed when its last claim is removed.
- A name that the daemon shows on its own layer-shell or X11 surface (see below) is not a stream:
  `RequestStream` and `HoldStream` return `InvalidArgs` for it. When such a screen appears with the
  name of an existing stream, that stream is closed and its claims are dropped.

Coverage is also tracked per claimant. An output counts as covered only when every claimant
reports it covered. `SetOutputCovered` from a client without a claim applies to the held claim.

A stream whose name starts with `preview:` is private: it never shows the content of the desktop
(`default`), only what its own `Library1.Apply` (or a draft) gives it, and an output that has not
shown content yet shows the first content applied to it without a transition.

A stream whose name is `lock:` followed by a screen name (`lock:DP-1`) is the lock screen of that
screen: it shows the lock choice of the screen (`SetLockWallpaper`, `SetLockPlaylist`), with the
placement, color, filter, quality and settings of the screen itself, and is left out wherever the
daemon counts real screens (spanning, calibration, the switcher, the history of use). Without a lock
choice it shows what the screen shows, but a client that follows the desktop asks for the screen's own
stream instead, so nothing is drawn twice; the Plasma wallpaper plugin does so (see `GetLock`).

Output names are trimmed and an empty name means `default` in `RequestStream`, `HoldStream`,
`ReleaseStream` and `SetOutputCovered`.
