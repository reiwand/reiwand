# Self-hosted server

`reiwand-server` is one binary that serves a signed package repository, a web catalog, accounts,
uploads with moderation, and device sign-in for one community or household. It keeps every byte
of state in one data directory, makes no outgoing connection the operator did not
configure, and sends no telemetry.

The specification, part by part:

- [Running](server/running.md): running.
- [Conventions and Discovery and health](server/conventions.md): conventions, discovery and health.
- [Repository](server/repository.md): repository.
- [Catalog and Accounts](server/catalog.md): catalog, accounts.
- [Uploads](server/uploads.md): uploads.
- [Moderation and administration](server/moderation-and-administration.md): moderation and administration.
- [Legal pages and Operating a public server](server/legal-pages.md): legal pages, operating a public server.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="running"></a>[Running](server/running.md#running)
- <a id="first-setup"></a>[First setup](server/running.md#first-setup)
- <a id="stored-settings"></a>[Stored settings](server/running.md#stored-settings)
- <a id="configuration"></a>[Configuration](server/running.md#configuration)
- <a id="data-directory"></a>[Data directory](server/running.md#data-directory)
- <a id="conventions"></a>[Conventions](server/conventions.md#conventions)
- <a id="discovery-and-health"></a>[Discovery and health](server/conventions.md#discovery-and-health)
- <a id="repository"></a>[Repository](server/repository.md#repository)
- <a id="several-repositories"></a>[Several repositories](server/repository.md#several-repositories)
- <a id="root-key-kept-by-the-server"></a>[Root key kept by the server](server/repository.md#root-key-kept-by-the-server)
- <a id="explore-repositories-public-page"></a>[Explore repositories (public page)](server/repository.md#explore-repositories-public-page)
- <a id="private-repositories-and-rights"></a>[Private repositories and rights](server/repository.md#private-repositories-and-rights)
- <a id="database-versions-and-backups"></a>[Database versions and backups](server/repository.md#database-versions-and-backups)
- <a id="explicit-values-in-the-database"></a>[Explicit values in the database](server/repository.md#explicit-values-in-the-database)
- <a id="online-authors"></a>[Online authors](server/repository.md#online-authors)
- <a id="catalog"></a>[Catalog](server/catalog.md#catalog)
- <a id="accounts"></a>[Accounts](server/catalog.md#accounts)
- <a id="device-enrollment"></a>[Device enrollment](server/catalog.md#device-enrollment)
- <a id="uploads"></a>[Uploads](server/uploads.md#uploads)
- <a id="moderation-and-administration"></a>[Moderation and administration](server/moderation-and-administration.md#moderation-and-administration)
- <a id="deleting"></a>[Deleting](server/moderation-and-administration.md#deleting)
- <a id="mail"></a>[Mail](server/moderation-and-administration.md#mail)
- <a id="legal-pages"></a>[Legal pages](server/legal-pages.md#legal-pages)
- <a id="operating-a-public-server"></a>[Operating a public server](server/legal-pages.md#operating-a-public-server)
