# Scripts, jobs and output

This page explains what `reiwand-ctl` prints, how jobs and secrets work, the exit codes, the
language of its messages, and the commands that work without a daemon.

## Output

- Commands that change something (`set`, `mute`, `covered`, `input grant`, `lib rate`,
  `playlist rm`…) confirm with a line what they did.
- Queries accept `--json` and print the daemon's answer as is. Without it they show tables to read,
  not for scripts.
- The CLI silences the noise of the graphics library (`libEGL warning`) with `EGL_LOG_LEVEL=fatal`
  unless the variable is already defined.

## Jobs

What launches a job prints its number and ends.

- With `--wait`, it shows the progress and the result.
- Ctrl-C stops waiting without stopping the job. `reiwand-ctl cancel <n>` stops it.
- On a question (such as installing from an expired repository), Ctrl-C leaves it unanswered and
  changes nothing.

A job that fails always says why. If the error carries no description, the daemon sends
`failed without giving a reason` with the name of the error. The app shows it in your language and
the phone also gives a translated reason.

## Passwords and keys

Passwords and keys of providers are asked for without echo, or read from the first line of the
input with `--secret-stdin`. They never go on the command.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | fine |
| 1 | error |
| 2 | incorrect usage, also when it is the daemon that rejects an argument (a setting or a rule that does not exist, a value out of range) |

`reiwand-ctl preset rm` accepts paths with `:` (`~/photos/a:b.png:Night`).

## Language

Error messages, the notices of `scene check` and the output of the commands come out in the system
language. `scripts/translations.sh check` fails if a fixed English text appears in
`apps/reiwand-ctl` without going through translation.

The help of `reiwand-ctl` comes out in the language of the desktop with the catalog
`po/<language>/reiwand-ctl.po`. That covers:

- `--help` and the `help` command of each group;
- each argument with its default value and its possible values;
- the headings *Usage*, *Arguments*, *Options* and *Commands*;
- the notices with which it rejects a badly written command (unexpected argument, invalid value,
  missing required argument, suggestions).

The names of commands, options and their values stay in English, so scripts keep working the same
in any language.

## Commands without a daemon

| Command | What it does |
|---------|--------------|
| `reiwand-ctl pack` | create, sign, check, update and validate `.wpak` packages |
| `reiwand-ctl repo` | keys, signing, publishing, rotation and verification of a repository |

Both work without a daemon. The password of a key is asked on the terminal or read from
`REIWAND_KEY_PASSWORD`.
