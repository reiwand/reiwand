# Accounts and the phone's repositories

This page covers accounts on repository servers, moving the configuration between devices, and the
phone's *Repositories* page with its online sources and updates.

## Accounts

In the phone's *Settings*, *Account* lists the accounts of the servers with the repositories where
each is used (`reiwand-ctl account ls`). You can stop using one on a repository or remove it.

An account is started when adding a private repository or when publishing, with a code that is
approved in the browser. Its access is stored encrypted with a key of Android's keystore that does
not leave the phone, and is only sent to the server of its repository.

```sh
reiwand-ctl account sign-in 'https://example.org/r/private'   # accounts per repository: sign-in, ls, use, unlink, rm
```

## Moving the configuration to another device

There is no synchronization over the network: what each computer has is local. To take the
configuration from one to another (repositories, providers, lists, favorites, ratings, tags and
settings), export it to a file and import it on the other, from phone to computer or the reverse.

| Where | Export | Import |
|-------|--------|--------|
| Computer | `reiwand-ctl export` | `import` |
| Phone, in *Account*, with the file picker | *Export the configuration* | *Import a configuration* |

The keys of providers only leave if you check it, with the encrypted file.

## The phone's *Repositories* page

The *Repositories* page gathers where wallpapers come from, with or without a server of your own.
It is what the *Explore repositories* button of the empty library and the *From the internet*
option of *Add* offer.

### *Pictures from the internet*

At the top are the *Pictures from the internet*, the providers on the phone:

- Bing picture of the day;
- NASA astronomy picture;
- Wikimedia picture of the day;
- Wallhaven search;
- folders of a WebDAV, Nextcloud or Immich server.

Each one:

- starts off, says who it connects with and what that party sees, and does not touch the network
  until you turn it on;
- when off, neither the download now nor the automatic one do anything;
- if the address it points to changes, turns off again and forgets its key;
- downloads only images, every six hours at most and with the network and battery rules of the
  settings (*Use mobile data in the background too*); *Download now* is requested by the person and
  goes without that wait;
- stores each one in the library with the tag `from-ID`, keeps the last *N* (it does not remove
  favorites, those in lists or the one set) and shows the credits.

### Certificates

Over HTTPS, the phone (sources, repositories and server of your own) trusts the public roots that
the program carries and the certificate authorities that you installed in Android's settings. It
reads them when opening the app and looks at them again on each new connection.

If a server presents a certificate that does not pass that check, the source says so under its name
instead of silently not downloading.

### Adding and exploring a repository

Below are the *Repositories*.

1. Paste the address of a repository in a single field. If it is private, it asks for an account.
   If its server uses a certificate of its own, it shows its fingerprint for you to confirm.
2. When adding it, *Explore* opens. It searches its packages and marks them if they are for adults
   (it hides them with *Hide mature content*).
3. *Explore* shows the preview of each one: one each time the row is seen, only from the repository
   you added, with a gray placeholder while it arrives. It does not request those for adults except
   with *Show mature content without blur*. Previews are stored up to 16 MiB and deleted with the
   repository.
4. It installs packages in the library, checked against the signed index of the repository.

A package installed this way is the same library item on the phone and on the computer.

## Updates

Once a day, the phone updates its indexes like the desktop:

- with the network and battery rules of the settings (*Use mobile data in the background too*);
- only if there are repositories;
- while *Check repositories for updates every day* (`repos.auto_refresh`, on by default as on the
  desktop, in the phone's settings) stays on.

Then:

- the tab shows a notice with how many packages have a new version;
- *Updates* lists them with *Update* (the new version takes the place of the old one, with its lists
  and settings);
- *Check for updates* does it right away.

### Automatic updates

With *Update repositories automatically* (`repos.auto_update`), packages with a new version are
updated by themselves after that check, with the same network and battery rules and all the checks
of an installation. It is off by default, because updating without asking changes what runs.

You find it:

- in the phone's settings, under the wallpapers;
- on the computer, in the settings next to the repositories;
- with `reiwand-ctl config repos.auto_update true`.

> **Note:** A version that asks for a new permission is not installed by itself. It stays in the
> list with a note that says what it asks for now.

There is never a notification of updates.
