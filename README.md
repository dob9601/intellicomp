# Intellicomp

> **Warning**
> Early in development

Generate cross-shell command-line completions based off of a YAML specification

The shell completion ecosystem right now is a mess. To summarise its current state:
- Completions typically aren't cross-shell compatible and thus often need to be defined individually for each shell.
  - In turn, this often means that completions will only work in particular shells. For example, certain tools may autocomplete in Bash but not in Fish, but not vice-versa
- Where completions have been generated across shells, they tend to be very rudimentary and not fully utilise their potential

Nuacomp aims to plug the gap by providing a simple, human-readable framework for creating intelligent completions that can run on any shell - without the need to duplicate effort for different shells.

## Installation

Install the binary with `cargo install nuacomp` (or `cargo install --path .` when cd'd into the repo if building from git)

### Bash

Add the following hook to the end of your `~/.bashrc` file:

```bash
eval "$(nuacomp hook bash)"
```

### Fish

Add the following hook to the end of your `~/.config/fish/config.fish` file:

```fish
nuacomp hook fish | source
```

## Roadmap

| Feature                             | Implemented |
| ----------------------------------- | :---------: |
| Bash Support                        |      ✔️     |
| Fish Support                        |             |
| Zsh Support                         |             |
| Enumerations                        |      ✔️     |
| Path Completions                    |      ✔️     |
| Ramged numerical completions        |             |
| Positional Arguments                |      ✔️     |
| Keyword Arguments                   |      ✔️     |
| Non-repeatable/Repeatable Arguments |      ✔️     |
| Argument shorthands/aliases         |             |
| Subcommands                         |             |
| Argument descriptions               |             |
| Mutually exclusive arguments        |             |
| Schema repository                   |      ✔️     |
| Automatic periodic schema updating  |             |
| Bash shell hook                     |      ✔️     |
| Fish shell hook                     |             |
| Zsh  shell hook                     |             |
| Clap integration                    |             |
| Cool demo GIFs in the README        |             |
