# Provider plugins

A provider plugin teaches Reiwand a new place to get wallpapers from: your own gallery, an art site
with an API. It is a small TypeScript program that asks a web service for photos and returns a list.
Reiwand downloads the files, shows the credit and rotates them like those of any built-in provider.
The full rules are in [the specification](../specs/provider-plugins.md); this page walks you through
the example in [`examples/provider-plugin`](examples/provider-plugin/).

```
provider-plugin/
  reiwand-provider.json   what the plugin is and what it may reach
  main.ts                 the list function
  lib/shape.ts            a module that turns an answer into items
```

## What the plugin may do

A plugin is shipped as readable source so that anyone can see what it does. When it is installed,
the user is shown every host it contacts and the secret it asks for, and says yes or no. After that:

- it reaches only the hosts it declares, over https, and only public addresses;
- it has no files, no processes and no other network;
- each call has 32 MiB of memory, 2 s of processor time and 60 s in all, with at most 32 requests.

A request that breaks a rule stops the call, even inside a `try`. A request that simply fails on the
network (a timeout, no answer) is an error you can catch.

## The manifest

[`reiwand-provider.json`](examples/provider-plugin/reiwand-provider.json) writes every field; a
missing one is refused with its path, as in every Reiwand format.

- `hosts`: the exact host names: here the API and the host of the image files. A file whose address
  is on another host is refused, so list the host of the files too.
- `secret`: the API key the user types when adding a source of the plugin, or `null` if the service
  needs none. Reiwand puts
  it in the `header` you name, after `prefix`, and sends it only to the hosts in `secret.hosts`. Your
  code never sees it. With `"required": false` the plugin also works without it.
- `settings`: what the user can choose when adding the source. `prefill` is what the form offers;
  an `int` carries `min` and `max`, a `choice` its `values`, and the other fields are `null` or `[]`.

## The code

`main.ts` exports `list`. It receives the settings, typed as declared, and `page`: `null` for the
first page, then whatever you returned as `next`.

```ts
export async function list({ settings, page }: { settings: Settings; page: string | null }) {
  const answer = await fetch(`https://api.example.org/v1/photos?${query}`, {
    headers: { Accept: "application/json" },
  });
  if (!answer.ok) throw new Error(`the gallery answered ${answer.status}`);
  const found = (await answer.json()) as Answer;
  ...
  return { items, next: found.next_page === null ? null : String(found.next_page) };
}
```

Each item writes all its fields: `id` (unique), `title`, `media` (`"image"` or `"video"`), `url`
(the file), `page` (the web page of the photo, https, or `null`), `author` and `license` (or `null`
when the service does not say). Give the author and licence whenever the service has them: Reiwand
shows the credit.

`fetch` takes `method` (`GET` or `POST`), `headers` and a text `body`, and answers with `status`,
`ok`, `url`, `headers.get(name)`, `text()` and `json()`. There is no `URL` or `URLSearchParams`:
build query strings with `encodeURIComponent`, as the example does.

Modules import each other by relative path with the `.ts` ending (`./lib/shape.ts`). Reiwand removes
the types when it installs the plugin and does nothing else, so write TypeScript that is JavaScript
once its types are gone: no `enum`, no `namespace` with values, no `constructor(private x)`, no
decorators, and `value as T` instead of `<T>value`. Such code is refused at install with its file,
line and column.

## Trying it

```
reiwand-ctl provider plugin install docs/authoring/examples/provider-plugin
reiwand-ctl provider add gallery plugin:example-gallery query=lakes
reiwand-ctl provider sync gallery --wait
```

`install` shows the hosts and the secret and asks before installing; installing it again after a
change shows it again. `reiwand-ctl provider ls` shows the error of a sync that failed, with the
file, line or field of the fault.
