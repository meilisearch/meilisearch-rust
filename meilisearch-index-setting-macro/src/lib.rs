use convert_case::{Case, Casing};
use proc_macro2::Ident;
use quote::quote;
use structmeta::{Flag, StructMeta};
use syn::{parse_macro_input, spanned::Spanned};

#[derive(Clone, StructMeta, Default)]
struct FieldAttrs {
    primary_key: Flag,
    displayed: Flag,
    searchable: Flag,
    distinct: Flag,
    filterable: Flag,
    sortable: Flag,
}

#[proc_macro_derive(IndexConfig, attributes(index_config))]
pub fn generate_index_settings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    let fields: &syn::Fields = match ast.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => {
            return proc_macro::TokenStream::from(
                syn::Error::new(ast.ident.span(), "Applicable only to struct").to_compile_error(),
            );
        }
    };

    let struct_ident = &ast.ident;

    let index_config_implementation = get_index_config_implementation(struct_ident, fields);
    proc_macro::TokenStream::from(quote! {
        #index_config_implementation
    })
}

fn get_index_config_implementation(
    struct_ident: &Ident,
    fields: &syn::Fields,
) -> proc_macro2::TokenStream {
    let mut primary_key_attribute = String::new();
    let mut distinct_key_attribute = String::new();
    let mut displayed_attributes = vec![];
    let mut searchable_attributes = vec![];
    let mut filterable_attributes = vec![];
    let mut sortable_attributes = vec![];

    let index_name = struct_ident
        .to_string()
        .from_case(Case::UpperCamel)
        .to_case(Case::Snake);

    let mut primary_key_found = false;
    let mut distinct_found = false;

    for field in fields {
        let attrs = field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("index_config"))
            .map(|attr| attr.parse_args::<FieldAttrs>().unwrap())
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .unwrap_or_default();

        // Check if the primary key field is unique
        if attrs.primary_key.value() {
            if primary_key_found {
                return syn::Error::new(
                    field.span(),
                    "Only one field can be marked as primary key",
                )
                .to_compile_error();
            }
            primary_key_attribute = field.ident.clone().unwrap().to_string();
            primary_key_found = true;
        }

        // Check if the distinct field is unique
        if attrs.distinct.value() {
            if distinct_found {
                return syn::Error::new(field.span(), "Only one field can be marked as distinct")
                    .to_compile_error();
            }
            distinct_key_attribute = field.ident.clone().unwrap().to_string();
            distinct_found = true;
        }

        if attrs.displayed.value() {
            displayed_attributes.push(field.ident.clone().unwrap().to_string());
        }

        if attrs.searchable.value() {
            searchable_attributes.push(field.ident.clone().unwrap().to_string());
        }

        if attrs.filterable.value() {
            filterable_attributes.push(field.ident.clone().unwrap().to_string());
        }

        if attrs.sortable.value() {
            sortable_attributes.push(field.ident.clone().unwrap().to_string());
        }
    }

    let primary_key_token: proc_macro2::TokenStream = if primary_key_attribute.is_empty() {
        quote! {
            ::std::option::Option::None
        }
    } else {
        quote! {
            ::std::option::Option::Some(#primary_key_attribute)
        }
    };

    let display_attr_tokens =
        get_settings_token_for_list(&displayed_attributes, "with_displayed_attributes");
    let sortable_attr_tokens =
        get_settings_token_for_list(&sortable_attributes, "with_sortable_attributes");
    let filterable_attr_tokens =
        get_settings_token_for_list(&filterable_attributes, "with_filterable_attributes");
    let searchable_attr_tokens =
        get_settings_token_for_list(&searchable_attributes, "with_searchable_attributes");
    let distinct_attr_token = get_settings_token_for_string_for_some_string(
        &distinct_key_attribute,
        "with_distinct_attribute",
    );

    quote! {
        #[::meilisearch_sdk::macro_helper::async_trait(?Send)]
        impl ::meilisearch_sdk::documents::IndexConfig for #struct_ident {
            const INDEX_STR: &'static str = #index_name;

            fn generate_settings() -> ::meilisearch_sdk::settings::Settings {
            ::meilisearch_sdk::settings::Settings::new()
            #display_attr_tokens
            #sortable_attr_tokens
            #filterable_attr_tokens
            #searchable_attr_tokens
            #distinct_attr_token
        }

         async fn generate_index<Http: ::meilisearch_sdk::request::HttpClient>(client: &::meilisearch_sdk::client::Client<Http>) -> std::result::Result<::meilisearch_sdk::indexes::Index<Http>, ::meilisearch_sdk::tasks::Task> {
            return client.create_index(#index_name, #primary_key_token)
                .await.unwrap()
                .wait_for_completion(&client, ::std::option::Option::None, ::std::option::Option::None)
                .await.unwrap()
                .try_make_index(&client);
            }
        }
    }
}

fn get_settings_token_for_list(
    field_name_list: &[String],
    method_name: &str,
) -> proc_macro2::TokenStream {
    let string_attributes = field_name_list.iter().map(|attr| {
        quote! {
            #attr
        }
    });
    let method_ident = Ident::new(method_name, proc_macro2::Span::call_site());

    if field_name_list.is_empty() {
        quote! {
            .#method_ident(::std::iter::empty::<&str>())
        }
    } else {
        quote! {
            .#method_ident([#(#string_attributes),*])
        }
    }
}

fn get_settings_token_for_string_for_some_string(
    field_name: &String,
    method_name: &str,
) -> proc_macro2::TokenStream {
    let method_ident = Ident::new(method_name, proc_macro2::Span::call_site());

    if field_name.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        quote! {
            .#method_ident(::std::option::Option::Some(#field_name))
        }
    }
}
