{%- if cookiecutter.use_shorthands_for_proc_macro_crates -%}
use proc_macro as {{ cookiecutter.__pm_crate_ident }};
use proc_macro2 as {{ cookiecutter.__pm2_crate_ident }};
{% endif -%}

{%- for pm in cookiecutter.macro_idents.proc_macro %}
#[proc_macro]
pub fn {{ pm }}(input: {{ cookiecutter.__pm_crate_ident }}::TokenStream) -> {{ cookiecutter.__pm_crate_ident }}::TokenStream {
    let input = {{ cookiecutter.__pm2_crate_ident }}::TokenStream::from(input);
    let output = {{ cookiecutter.__macro_core_crate_ident }}::{{ pm }}(input);
    {{ cookiecutter.__pm_crate_ident }}::TokenStream::from(output)
}
{% endfor -%}

{% for pm in cookiecutter.macro_idents.proc_macro_derive %}
#[proc_macro_derive({{ pm.derive_name }}{% if pm.attributes %}, attributes({% for attr in pm.attributes %}{{ attr }}{{ ", " if not loop.last }}{% endfor %}){% endif %})]
pub fn {{ pm.fn_name }}(input: {{ cookiecutter.__pm_crate_ident }}::TokenStream) -> {{ cookiecutter.__pm_crate_ident }}::TokenStream {
    let input = {{ cookiecutter.__pm2_crate_ident }}::TokenStream::from(input);
    let output = {{ cookiecutter.__macro_core_crate_ident }}::{{ pm.fn_name }}(input);
    {{ cookiecutter.__pm_crate_ident }}::TokenStream::from(output)
}
{% endfor -%}

{% for pm in cookiecutter.macro_idents.proc_macro_attribute %}
#[proc_macro_attribute]
pub fn {{ pm }}(attr: {{ cookiecutter.__pm_crate_ident }}::TokenStream, item: {{ cookiecutter.__pm_crate_ident }}::TokenStream) -> {{ cookiecutter.__pm_crate_ident }}::TokenStream {
    let attr = {{ cookiecutter.__pm2_crate_ident }}::TokenStream::from(attr);
    let item = {{ cookiecutter.__pm2_crate_ident }}::TokenStream::from(item);
    let output = {{ cookiecutter.__macro_core_crate_ident }}::{{ pm }}(attr, item);
    {{ cookiecutter.__pm_crate_ident }}::TokenStream::from(output)
}
{% endfor -%}
