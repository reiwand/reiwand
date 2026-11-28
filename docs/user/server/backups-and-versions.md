# Backups and data from another version

Everything the server keeps lives in its data folder, so a backup is a copy of that data. This
page explains how to make and restore a backup, and what happens with data made by another
version.

## Making a backup

```sh
docker compose exec reiwand reiwand-server backup
```

This leaves in `/data/backups/<date>` a consistent snapshot of the database (`VACUUM INTO`) with:

- the packages, thumbnails and private files it names,
- the keys and the repository,
- the SHA-256 sum of each file.

## Restoring a backup

Restore with the server stopped:

1. Stop the server.
2. Run the restore with the backup folder.
3. Start the server again.

```sh
docker compose stop reiwand                                   # the backups are in ./data/backups
docker compose run --rm reiwand restore /data/backups/<date> --force
docker compose up -d
```

The restore checks the sums and the integrity of the database, and signs a new index so that
clients do not see an earlier version.

## Data from another version

The server only knows the data format of its version.

If the database has another schema, it refuses to start, does not touch it and says:

> the database has schema N but this program reads only M: it was made by another version, which
> this version does not read; the database was left untouched

If saved settings are missing, it says:

> the saved settings lack "KEY"; they were made by another version, which this version does not
> read

There is no program that converts the data: a change that breaks the format is a major version.

> **Note:** To go back to an earlier version, restore the backup you made with it.

## Packages from another version

Published packages are stored as their authors uploaded them; their signatures belong to each
author. A client that downloads one with a manifest of another version rejects it. The author
makes it again in the current format, signs it and uploads a new version.
