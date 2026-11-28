# JSON model

Every reply below carries `model` (currently `1`) and `serial` first. Every field is always
written, with no omission (`null` for none, `[]` and `{}` for empty): the daemon never skips a field,
whatever its value. A client reads a reply strictly: a missing field is an error that names it
(`item`, `entries[2].item.title`), and no client fills in a value of its own. Clients ignore fields
they do not know, and read enum values they do not know as unknown.

The JSON model, part by part:

- [The rules of the model](json-model/rules.md): explicit fields, versions and what the state fields mean.
- [State, jobs and the library](json-model/state-jobs-library.md): the types of the state, outputs, settings fields, jobs and library items.
- [Lists, providers, inputs and accounts](json-model/lists-providers-inputs.md): the types of playlists, profiles, presets, providers, repositories, inputs and accounts, and notes on some fields.
