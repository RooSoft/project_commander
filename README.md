# Project Commander

Recursive fuzzy search for git projects starting from a given parent folder

## How to build

It's a rust app, so

```bash
cargo build
```

and a debug version of the app should end up in the `target/debug` folder.

## Usage

```
Usage: pc [OPTIONS]

Options:
-n, --name <NAME>  Name of a project to search for
-h, --help         Print help
-V, --version      Print version
```

When called without argument, a CLI interface should appear with a list of all
git projects from the current folder. Pressing `/` brings the search bar.
