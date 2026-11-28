# Signed repositories

A repository is a static tree of files that any web server, file share or `file://` path can
serve. Every file the client trusts is either signed with minisign (Ed25519, prehashed `ED`
signatures only) or pinned by SHA-256 from a signed file. Mirrors and servers are never
trusted. The reference implementation lives in `reiwand-format` (verification always, key
generation and signing behind the `sign` feature).

The specification, part by part:

- [Keys and roles, Signature files, Layout and Identity and URL](repo/keys-and-roles.md): keys and roles, signature files, layout, identity and url.
- [Formats and Package signatures](repo/formats.md): formats, package signatures.
- [Online authors, Operator workflow, Client update, Package installation, Broken repositories and Client storage](repo/online-authors.md): online authors, operator workflow, client update, package installation, broken repositories, client storage.
- [Bundled example repository, Network interface and Adding a repository in the desktop app](repo/bundled-example-repository.md): bundled example repository, network interface, adding a repository in the desktop app.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="keys-and-roles"></a>[Keys and roles](repo/keys-and-roles.md#keys-and-roles)
- <a id="signature-files"></a>[Signature files](repo/keys-and-roles.md#signature-files)
- <a id="layout"></a>[Layout](repo/keys-and-roles.md#layout)
- <a id="identity-and-url"></a>[Identity and URL](repo/keys-and-roles.md#identity-and-url)
- <a id="formats"></a>[Formats](repo/formats.md#formats)
- <a id="package-signatures"></a>[Package signatures](repo/formats.md#package-signatures)
- <a id="online-authors"></a>[Online authors](repo/online-authors.md#online-authors)
- <a id="operator-workflow"></a>[Operator workflow](repo/online-authors.md#operator-workflow)
- <a id="client-update"></a>[Client update](repo/online-authors.md#client-update)
- <a id="package-installation"></a>[Package installation](repo/online-authors.md#package-installation)
- <a id="broken-repositories"></a>[Broken repositories](repo/online-authors.md#broken-repositories)
- <a id="client-storage"></a>[Client storage](repo/online-authors.md#client-storage)
- <a id="bundled-example-repository"></a>[Bundled example repository](repo/bundled-example-repository.md#bundled-example-repository)
- <a id="network-interface"></a>[Network interface](repo/bundled-example-repository.md#network-interface)
- <a id="repositories-with-an-account"></a>[Repositories with an account](repo/bundled-example-repository.md#repositories-with-an-account)
- <a id="adding-a-repository-in-the-desktop-app"></a>[Adding a repository in the desktop app](repo/bundled-example-repository.md#adding-a-repository-in-the-desktop-app)
