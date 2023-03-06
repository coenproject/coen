# Coen

A RegEx-based transpiler for markdown-like languages.

## Installation

Coen is available in [crates.io](https://crates.io/crates/coen) and can be installed using `Cargo`:

```
cargo install coen
```

## Usage

Use a template to create documents (for example, clone [this one](https://github.com/coenproject/coen-template-tex) for LaTeX) and add/modify your contents in the `*.coen` files.

### Setting variables

You can set variables using the `set` command:

```
!set variable value
```

### Adding definitions

You can add definitions using the `def` command. Groups are captured with `(?P<group_name>.+)` and can be replaced using `$group_name`.

```
!def blue\s(?P<name>.+) gray $name
```

This one, for example, changes every instance of `blue *` into `gray *`.

## Transpilation

Pass the path to the root file and supply an optional target location:

```
coen root.coen --target tgt/target.tex
```

You can also set the target location within the coen file with:

```
!set TARGET tgt/target.tex
```

Passing the target parameter in the command line will override this value.
