# Repositories and accounts

A server hosts one or several repositories, and people use them with accounts. This page explains
repository addresses, public and private repositories, accounts, and how to sign in, also without
a browser.

## Repositories

A server hosts one or several repositories, created with `reiwand-server repo init --name N` or
*Repositories* in the web administration.

- Each one has its simple address, `https://server/r/<name>`. It is pasted as is into the app or
  with `reiwand-ctl remote add`: no fingerprints or parameters.
- The main page of the server, `/`, is "Explore repositories". It lists the repositories with
  their address to copy and the "Open in Reiwand" button.
- A **public** repository is added without asking for anything.
- A **private** one asks for an account when connecting. The administrator gives each person read
  or publish rights per repository (*People*).
- A repository can accept uploads from any account (for review) or only from its members.

Pasting the address of a server (without `/r/<name>`) does not list repositories: it says it is a
server and names the addresses of its public repositories.

## Accounts

An **account** belongs to a person on a server and is linked to the repositories where it is used.
One person can have several.

- You sign in with a code that is approved in the browser.
- The session is stored in the system keyring. Without a keyring, `reiwand-ctl provider store
  file` stores it in a private file in the data folder, after a notice.
- The access is only sent to the server of its repository.
- There is no synchronization: the library and settings of each computer are local.

If you add a repository of a server where you already have an account, the app asks whether you
use that account or another. In the terminal, use `account use` or `account sign-in`.

When publishing, the target repository is chosen. If there is no account linked to it, it is
asked for there, whether it is public or private.

## Terminal commands

```sh
reiwand-ctl remote add https://wallpapers.example.org/r/main     # public: it is added
reiwand-ctl remote add https://192.168.1.10:8443/r/main      # home server with its own certificate: the first time it shows its fingerprint
reiwand-ctl remote add https://192.168.1.10:8443/r/main --trust-certificate <fingerprint>   # … and you confirm it
reiwand-ctl account sign-in https://wallpapers.example.org/r/private   # shows a code and an address; approve it in the browser
reiwand-ctl account ls                                       # your accounts and the repositories where each one is used
reiwand-ctl account use https://wallpapers.example.org/r/other a1  # use that account on another repository of the same server
reiwand-ctl account unlink https://wallpapers.example.org/r/other # stop using it there; the account is kept
reiwand-ctl account rm a1                                    # forget it on this computer, with its stored access
reiwand-ctl server upload wallpaper.wpak                     # upload a package with the account of the repository
reiwand-ctl server upload wallpaper.wpak --unlisted          # … hidden: out of the catalog, installable with its link
reiwand-ctl server upload wallpaper.wpak --repo https://wallpapers.example.org/r/other   # … in that repository, with the account linked to it
reiwand-ctl server packages --repo https://wallpapers.example.org/r/other   # your uploads in that repository, with their state and visibility
reiwand-ctl server uploads                                   # the upload sessions (in progress or finished); --cancel ID cancels one
reiwand-ctl server author-key list                           # the keys with which you sign your packages online
reiwand-ctl server author-key add author.pub                 # registers a key; `rotate`, `remove` change or remove it
reiwand-ctl server visibility org.example.wallpaper public       # changes the visibility of a package or a version
reiwand-ctl server withdraw org.example.wallpaper            # withdraws a package of yours for good, after asking
reiwand-ctl export config.reiwand                            # the configuration of this computer in a file (see below)
reiwand-ctl import config.reiwand                            # … and it is added on another computer
```

For export and import, see [Moving the configuration](moving-configuration.md).

> **Note:** With more than one repository with an account, `server packages`, `uploads`,
> `author-key` and `visibility` ask for `--repo ADDRESS` (the address or the id of the
> repository). Without it, they fail naming the addresses to choose from.

## Accounts in the app

In the app, *Account* shows on each account, per repository:

| Section | What it does |
|---------|--------------|
| *My uploads* | State and visibility, which is changed there; unfinished sessions are cancelled |
| *Author keys* | List, register from a file, hand over and remove |

An account that must sign in again says so on its row. When publishing or voting with it, the app
offers *Sign in again* right there.

## Your first account, without a browser on the computer

This assumes the administrator account already exists and `cookies` is its session, from
[Installing the server](install.md).

1. Without a system keyring, store sessions in a file.
2. Sign in: the command prints the code and the address, and waits.
3. Approve the code with the administrator session. With a browser, open the address and type the
   code instead.
4. Check that the account is there.

```sh
reiwand-ctl provider store file                           # only without a system keyring
reiwand-ctl account sign-in http://127.0.0.1:8080/r/main  # prints the code ABCD-EFGH and the address, and waits
curl -sS -b cookies -H 'content-type: application/json' \
  -d '{"user_code": "ABCD-EFGH", "approve": true}' \
  http://127.0.0.1:8080/api/v1/auth/device/approve     # {"approved":true}; with a browser, open the address and type the code
reiwand-ctl account ls                                    # the account, with the repository where it is used
```

`http://` is only valid for this same computer. A server on another computer needs HTTPS, with
its own certificate or with a proxy (see [HTTPS](https.md)).

## Security of sign-in codes

> **Warning:** Only approve a code shown on the screen of your own computer. Whoever sends you a
> code is trying to get into your account.

- Approving requires a web session of the account, not the access of a computer. So whoever
  steals the access of one computer cannot add others.
- Changing the password on the web closes the other sessions and deletes the tokens.
