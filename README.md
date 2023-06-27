# proc_macro_template

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Cookiecutter](https://github.com/cookiecutter/cookiecutter) template for creating [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) in [Rust](https://www.rust-lang.org/) programming language.

## Requirements

In order to use the template, you need to have `cookiecutter` installed. If needed, check out the [Installation](https://cookiecutter.readthedocs.io/en/stable/installation.html) section of the docs.

## Usage

With `cookiecutter` installed,

```console
python -m cookiecutter gh:JohnScience/proc_macro_template
```

It will prompt you to fill out the cookiecutter variables:

* `crate_name` - the name of the Rust crate that will be the procedural macro library. Some examples of crate names are [`strum`](https://crates.io/crates/strum), [`cfg-if`](https://crates.io/crates/cfg-if), and [`rand_chacha`](https://crates.io/crates/rand_chacha).
* `crate_name_sep` - the separator that should be used for concatenation of parts in crate names: either `_` (underscore) or `-` (dash).
* `macro_idents` - identifiers that will be used for creation of macros. They are collected into an object with complex schema. Check the [default value in `cookiecutter.json`](https://github.com/JohnScience/proc_macro_template/blob/main/cookiecutter.json#L4-L23) for an example.
* `use_shorthands_for_proc_macro_crates` - whether use identifiers `pm` and `pm2` instead of [`proc_macro`](https://doc.rust-lang.org/proc_macro/) and [`proc_macro2`](https://crates.io/crates/proc-macro2).

For quick prototyping you can use the default values from `cookiecutter.json` by appending the [`--no-input`](https://cookiecutter.readthedocs.io/en/1.7.0/advanced/cli_options.html#cmdoption-no-input) flag:

```console
python -m cookiecutter gh:JohnScience/proc_macro_template --no-input
```

## About the template

The template was inspired by by Carl M. Kadie's article ["Nine Rules for Creating Procedural Macros in Rust"](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff).

The template consists of the `<crate_name>_core` crate that implements the functionality and `<crate_name>` crate that exposes the usable procedural macros.

## Generated project example

Example of a project generated using this cookiecutter is available [here](https://github.com/JohnScience/proc_macro_template_example).

## On (in)stability of `cookiecutter.json`

As mentioned in [issue #1 "Making input easier"](https://github.com/JohnScience/proc_macro_template/issues/1), the author believes the input could be made easier. Consequently, `cookiecutter.json` can change.

## Notes

This template was created before the author recalled that there's also [`cargo-generate`](https://crates.io/crates/cargo-generate), which would allow Rust developers to use only the already available tooling.
