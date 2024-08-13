# Command-Line Help for `hy`

This document contains the help content for the `hy` command-line program.

**Command Overview:**

* [`hy`↴](#hy)
* [`hy markdown`↴](#hy-markdown)
* [`hy init`↴](#hy-init)
* [`hy check`↴](#hy-check)
* [`hy generate`↴](#hy-generate)
* [`hy server-mode`↴](#hy-server-mode)

## `hy`

hay cli

**Usage:** `hy [COMMAND]`

###### **Subcommands:**

* `markdown` — [STABLE] print markdown doc of hay to std out
* `init` — [STABLE] creates an example config
* `check` — [PREVIEW] checks all endpoints
* `generate` — [NOT IMPLEMENTED] generates ci actions to create that checks endpoints and updates website
* `server-mode` — [NOT IMPLEMENTED] lets the ci run in server mode, continously checks endpoints



## `hy markdown`

[STABLE] print markdown doc of hay to std out

**Usage:** `hy markdown`



## `hy init`

[STABLE] creates an example config

**Usage:** `hy init`



## `hy check`

[PREVIEW] checks all endpoints

**Usage:** `hy check [OPTIONS] [FILE]`

###### **Arguments:**

* `<FILE>`

###### **Options:**

* `-j`, `--json`

  Default value: `false`



## `hy generate`

[NOT IMPLEMENTED] generates ci actions to create that checks endpoints and updates website

**Usage:** `hy generate [OPTIONS]`

###### **Options:**

* `-c`, `--ci`

  Default value: `true`
* `-w`, `--web`

  Default value: `false`



## `hy server-mode`

[NOT IMPLEMENTED] lets the ci run in server mode, continously checks endpoints

**Usage:** `hy server-mode`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

