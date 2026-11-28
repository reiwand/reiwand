# Bundled example repository, Network interface and Adding a repository in the desktop app

On this page: bundled example repository, network interface, adding a repository in the desktop app.

## Bundled example repository

The example wallpapers that come with the program are a repository that is already added, so that
a new installation has something to explore without any server. The packages are the `.wpak` files
every way of installing carries: `reiwand/examples/` under the user data folder (the
AppImage) or under `/usr/share` or `/app/share` (RPM, Flatpak); the phone builds them from
the scene folders of its assets.

`reiwand_format::bundled(repo, keys, name, packages, now)` makes and keeps that repository. No key
is in the source tree or in a package: the first run creates two passwordless minisign keys (root
and online) in a folder of mode `0700` beside the repository, signs `root/1.json`, adds the
packages and publishes. The repository is local, so its trust is the folder the daemon owns, like
the library; its address is the plain `file://` one that
`status` prints. It is signed again on later runs, never at build time, because a signed index
cannot outlive `entry_max_days` (90 for this repository) and a build is older than that on
most machines. Each run adds the packages whose `id` and version are not in it, publishes a new
index when anything changed or fewer than 45 days remain, and writes a new root (signed by the
same key) when fewer than 90 days remain of its 1096. A package whose version is already
published with other content is refused, so a change to an example bumps its version.

- Desktop: the daemon runs it at startup and once a day, adds the repository when it is not added
  and refreshes it when it changed. `examples-repo.json` in the data folder remembers its
  `repo_id` and whether the user removed it. After `Repos1.Remove` it is not added again, and
  `Repos1.AddExamples` (`reiwand-ctl remote examples`) adds it again; the repository folder and the
  keys are kept, so it keeps its `repo_id`. `Repos1.List` marks it `bundled: true`; a screen uses
  that, not the name, to give it its own name and icon. A program update only has to replace the
  packages in the shipped folder.
- Phone: `examples_due` and `examples_repo` do the same from the assets; the app asks at start
  and the repository is built again only when the app was updated, apart from renewing its index
  without unpacking anything. The repository list marks it `bundled`. Removing it is remembered;
  the *Add the example scenes* action adds it again, besides importing the scenes as before.
- It is never synced, because only repositories added with `https://` or loopback `http://` are.

## Network interface

The library does no networking. Callers implement `Fetch::get(base, path, max)`, returning
at most `max + 1` bytes or `None` when the file does not exist, and may override
`Fetch::copy` to stream packages. `FileFetch` serves `file://` URLs and plain paths. Requests
carry no cookies or identifiers.

`reiwand_providers::HttpFetch` is the blocking HTTP implementation: a generic `User-Agent`, no
cookies or `Referer`, `https://` or `http://` to a loopback host only (redirects included, never
from HTTPS to HTTP), redirects only to the origins of the bases it was asked to read or given with
`within(bases)` (a repository's URL and the mirrors of its signed root), `file://` URLs and plain
paths copied locally in chunks of 64 KiB, a 10 s connect timeout, and an abort
(`io::ErrorKind::TimedOut`) when less than 8 KiB arrive in 30 s. The transfer runs on its own
thread; a shared cancel flag, checked for every chunk and at least every 100 ms while waiting for
the server, fails the transfer with `io::ErrorKind::Interrupted` at once, local copies included.

### Repositories with an account

`HttpFetch::account(base, token)` adds an `Authorization: Bearer` to requests only when the URL
matches the origin and the path prefix of the repository the account is linked to (the longest
prefix wins). It is never sent to a mirror, to another origin or to a redirect to another origin. A
`401` with `account_needed` on the signed index makes the add fail with `AccountNeeded` (`dbus.md`),
whose message lists the accounts known on that server. Tokens live in the secret store, one
`account <id>` item per account; `accounts.json` stores none.

## Adding a repository in the desktop app

A single button, "Add a repository", always visible in Explore and at the start of the Repositories
page. It opens a box with a field for the address and "Add" (Enter in the field does the same; Esc
closes). The daemon's answer decides what follows:

- Public: it is added, the box closes and a notice says so.
- `AccountNeeded`: the box asks for an account. It lists "Use the account X" for each known account
  of that server (`Accounts1.List`; arrows and Enter choose; `Accounts1.Use` and it is added again)
  and "Use another account" (or "Sign in" if there is none), which calls `Accounts1.SignIn`, shows
  the code and the approval address and adds only when the sign-in job ends.
- `CertificateUnknown`: "This server uses its own certificate", with the fingerprint and "Trust and
  add" (`Repos1.AddTrusting`).
- `NotARepository`: "This is the address of a server"; it lists the repositories carried by the
  `output` of the failed task (name and address, without reading the error text), each one is
  chosen with a click or Enter and added.
- `CertificateChanged` and `SignatureChanged`: notice, nothing is added.

The Repositories page marks with "Needs a look" the repositories with `signature_changed` or
`certificate_changed` and says the account each private repository uses (`Accounts1.List`). The
`reiwand://add?url=<address>` scheme opens Explore with the box and the address filled in; an empty
address, one with control characters or of more than 2048 bytes is ignored.
