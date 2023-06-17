# Proc macro project `{{ cookiecutter.crate_name }}`

The main goal of this project is to enable the [`{{ cookiecutter.crate_name }}`](https://crates.io/crates/{{ cookiecutter.crate_name }}) [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros) library for [Rust](https://www.rust-lang.org/) programming language.

From technical standpoint, this project is a [Cargo Workplace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) that consists of two crates:

* `{{ cookiecutter.crate_name }}` - the proc macro library that *exposes* the procedural macros.
* `{{ cookiecutter.__macro_core_crate }}` - the library that *implements* the procedural macros in a "debaggable", unit-testable way.

## License

Licensed under {{ cookiecutter.license }}.

## Credits

The skeleton of this project was generated with [`proc_macro_template`](https://github.com/JohnScience/proc_macro_template), which in turn was inspired by Carl M. Kadie's ["Nine Rules for Creating Procedural Macros in Rust"](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff).
