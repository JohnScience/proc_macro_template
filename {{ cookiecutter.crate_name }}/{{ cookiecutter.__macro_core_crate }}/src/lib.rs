// Consider removing the attributes below after prototyping
#![allow(unused_imports)]
#![allow(unused_variables)]

use quote::quote;
{%- if  cookiecutter.use_shorthands_for_proc_macro_crates %}
use proc_macro2 as {{ cookiecutter.__pm2_crate_ident }};
{% endif -%}

{%- for pm in cookiecutter.macro_idents.proc_macro %}
pub fn {{ pm }}(input: {{ cookiecutter.__pm2_crate_ident }}::TokenStream) -> {{ cookiecutter.__pm2_crate_ident }}::TokenStream {
    todo!()
}
{% endfor -%}
{%- for pm in cookiecutter.macro_idents.proc_macro_derive %}
pub fn {{ pm.fn_name }}(input: {{ cookiecutter.__pm2_crate_ident }}::TokenStream) -> {{ cookiecutter.__pm2_crate_ident }}::TokenStream {
    todo!()
}
{% endfor -%}
{%- for pm in cookiecutter.macro_idents.proc_macro_attribute %}
pub fn {{ pm }}(attr: {{ cookiecutter.__pm2_crate_ident }}::TokenStream, item: {{ cookiecutter.__pm2_crate_ident }}::TokenStream) -> {{ cookiecutter.__pm2_crate_ident }}::TokenStream {
    todo!()
}
{% endfor %}
#[cfg(test)]
mod tests {
    use super::*;
    
    fn assert_tokens_eq(expected: &{{ cookiecutter.__pm2_crate_ident }}::TokenStream, actual: &{{ cookiecutter.__pm2_crate_ident }}::TokenStream) {
        let expected = expected.to_string();
        let actual = actual.to_string();
    
        if expected != actual {
            println!(
                "{}",
                colored_diff::PrettyDifference {
                    expected: &expected,
                    actual: &actual,
                }
            );
            println!("expected: {}", &expected);
            println!("actual  : {}", &actual);
            panic!("expected != actual");
        }
    }
    {% for pm in cookiecutter.macro_idents.proc_macro %}
    #[test]
    pub fn test_{{ pm }}() {
        let before = quote! {
            // Function-like macro input. Nearly arbitrary AST.
        };

        let expected = quote! {
            // Function-like macro output. The invocation is replaced with this.
        };

        let actual = {{ pm }}(before);

        assert_tokens_eq(&actual, &expected);
    }
    {% endfor -%}
    {%- for pm in cookiecutter.macro_idents.proc_macro_derive %}
    #[test]
    pub fn test_{{ pm.fn_name }}() {
        let before = quote! {
            // Derive macro input: struct, enum, or union.
        };

        let expected = quote! {
            // Derive macro output. It is appended to the resulting AST.
        };

        let actual = {{ pm.fn_name }}(before);

        assert_tokens_eq(&actual, &expected);
    }
    {% endfor -%}
    {%- for pm in cookiecutter.macro_idents.proc_macro_attribute %}
    #[test]
    pub fn test_{{ pm }}() {
        let attr = quote! {
            // Attribute macro input. For example, `target_os = "linux"` in
            // `#[cfg(target_os = "linux")]`.
            // https://doc.rust-lang.org/reference/attributes.html
        };

        let item = quote! {
            // Attribute macro input: an item.
            // https://doc.rust-lang.org/reference/items.html
        };

        let expected = quote! {
            // Attribute macro output. It replaces the input item.
        };

        let actual = {{ pm }}(attr, item);

        assert_tokens_eq(&actual, &expected);
    }
    {% endfor -%}
}