# Uploads and the catalog

Accounts upload `.wpak` packages to a repository, and the server shows them in a web catalog
without JavaScript. This page covers visibility, mature packages, the catalog, featured packages
and votes.

## Public or hidden uploads

When uploading a package you choose whether it is public or hidden: in the upload dialog of
*Repositories*, on the server web, or with `--unlisted`.

- A hidden package is published and signed the same.
- The catalog and the search do not show it.
- Whoever has its link or its id sees and installs it.

> **Warning:** Hidden is not private. It is still in the signed files of the repository, which
> anyone can download and read, with its title and description. What is private goes in a private
> repository.

## The Account page of the web

The *Account* page of the web lets you:

- list your uploads and change the visibility of each version,
- see and cancel the upload sessions left half done,
- register, change (with the signature of `pack rotate-key`) and remove online author keys.

See [Author keys and package tools](author-keys.md) for the keys.

## Mature packages

The server accepts mature packages that declare so in their manifest.

- They carry the `mature` tag in the catalog, in the search (`content=mature`, `facet=mature`)
  and in the API.
- The web blurs their thumbnails until you choose to view them, one per package and without
  JavaScript.
- The web does not store the choice, so it blurs them again on each visit.

## The web catalog

| Feature | Behavior |
|---------|----------|
| Filters | By rating and by facet |
| Order | Newest, title or most votes |
| Pages | Pages of 24 with *Previous* and *Next* |
| Warnings | Warns on the page of a photosensitive package |

The search, the filters, the order and the page are all in the address
(`/r/main/?q=sky&sort=title&page=2`), so any page can be bookmarked or shared.

## Featured packages

In the catalog of *Repositories* (and in *Explore* on the phone) the packages featured by the
repository operator come first, in their own section.

The operator signs them in the repository with:

- `reiwand-ctl repo feature REPO ID --note …`,
- the *Static repository* tab of the desktop app (see [Static repositories](static-repositories.md)),
- or `/admin/featured` on the server.

## Votes

With an account on the server, each package of that server's repository shows its votes for and
against, with your vote marked, and buttons to vote.

```sh
reiwand-ctl remote vote ID up|down|none
reiwand-ctl remote votes ID…
```

Both also accept `REPO/ID`.

- Packages of other repositories show no votes, and their ids are never sent to the server.
- The server counts one vote per account and package.
- It stores only the counts and your own vote, never a list of who voted or of who installed what.
- The setting *Show the number of votes* (`repos.vote_counts`, also in the phone settings) stops
  requesting and showing the counts.

Your local ratings in the library remain private and only leave in an exported file.
