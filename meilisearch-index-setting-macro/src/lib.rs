use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro_derive(Document, attributes(document))]
pub fn generate_index_settings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    let fields: &syn::Fields = match ast.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => {
            return proc_macro::TokenStream::from(
                syn::Error::new(ast.ident.span(), "Applicable only to struct").to_compile_error(),
            )
        }
    };

    let struct_name = &ast.ident;

    let document_implementation = get_document_implementation(struct_name, fields);
    proc_macro::TokenStream::from(quote! {
        #document_implementation
    })
}

fn get_document_implementation(
    struct_ident: &syn::Ident,
    fields: &syn::Fields,
) -> proc_macro2::TokenStream {
    let mut attribute_count: std::collections::HashMap<String, i32> =
        std::collections::HashMap::new();
    let mut primary_key_attribute: String = "".to_string();
    let mut distinct_key_attribute: String = "".to_string();
    let mut displayed_attributes: Vec<String> = vec![];
    let mut searchable_attributes: Vec<String> = vec![];
    let mut filterable_attributes: Vec<String> = vec![];
    let mut sortable_attributes: Vec<String> = vec![];
    let document_name = struct_ident.to_string().to_lowercase();

    for field in fields {
        let attribute_list_result = extract_all_attr_values(&field.attrs, &mut attribute_count);

        match attribute_list_result {
            Ok(attribute_list) => {
                for attribute in attribute_list {
                    match attribute.as_str() {
                        "displayed" => {
                            displayed_attributes.push(field.ident.clone().unwrap().to_string())
                        }
                        "searchable" => {
                            searchable_attributes.push(field.ident.clone().unwrap().to_string())
                        }
                        "filterable" => {
                            filterable_attributes.push(field.ident.clone().unwrap().to_string())
                        }
                        "sortable" => {
                            sortable_attributes.push(field.ident.clone().unwrap().to_string())
                        }
                        "primary_key" => {
                            primary_key_attribute = field.ident.clone().unwrap().to_string()
                        }
                        "distinct" => {
                            distinct_key_attribute = field.ident.clone().unwrap().to_string()
                        }
                        random => {
                            return syn::Error::new(
                                field.ident.span(),
                                format!("Property `{}` does not exist for type `document`", random),
                            )
                            .to_compile_error()
                        }
                    }
                }
            }
            Err(e) => {
                return e;
            }
        }
    }

    if primary_key_attribute.is_empty() {
        return syn::Error::new(struct_ident.span(), "There should be exactly 1 primary key")
            .to_compile_error();
    }

    let display_attr_tokens =
        get_settings_token_for_list(&displayed_attributes, "with_displayed_attributes");
    let sortable_attr_tokens =
        get_settings_token_for_list(&sortable_attributes, "with_sortable_attributes");
    let filterable_attr_tokens =
        get_settings_token_for_list(&filterable_attributes, "with_filterable_attributes");
    let searchable_attr_tokens =
        get_settings_token_for_list(&searchable_attributes, "with_searchable_attributes");
    let distinct_attr_token =
        get_settings_token_for_string(&distinct_key_attribute, "with_distinct_attribute");

    quote! {
        impl meilisearch_sdk::documents::Document for #struct_ident {
            fn generate_settings(&self) -> meilisearch_sdk::settings::Settings {
            meilisearch_sdk::settings::Settings::new()
            #display_attr_tokens
            #sortable_attr_tokens
            #filterable_attr_tokens
            #searchable_attr_tokens
            #distinct_attr_token
        }

        async fn generate_index(client: &meilisearch_sdk::client::Client) -> std::result::Result<meilisearch_sdk::indexes::Index, meilisearch_sdk::errors::Error> {
            client.create_index(#document_name, Some(#primary_key_attribute))?
                .wait_for_completion(client)?
                .try_make_index(client)
            }
        }
    }
}

fn extract_all_attr_values(
    attrs: &[syn::Attribute],
    attribute_count: &mut std::collections::HashMap<String, i32>,
) -> std::result::Result<Vec<String>, proc_macro2::TokenStream> {
    let mut value: Vec<String> = vec![];
    for attr in attrs {
        if let std::result::Result::Ok(syn::Meta::List(list)) = attr.parse_meta() {
            if list.path.is_ident("document") {
                for nested_meta in list.nested {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested_meta {
                        for segment in path.segments {
                            value.push(segment.ident.to_string());
                            *attribute_count
                                .entry(segment.ident.to_string())
                                .or_insert(0) += 1;

                            if segment.ident == "primary_key"
                                && attribute_count.get("primary_key").unwrap_or(&0) > &1
                            {
                                return std::result::Result::Err(
                                    syn::Error::new(
                                        segment.ident.span(),
                                        "primary_key already exists",
                                    )
                                    .to_compile_error(),
                                );
                            }
                            if segment.ident == "distinct"
                                && attribute_count.get("distinct").unwrap_or(&0) > &1
                            {
                                return std::result::Result::Err(
                                    syn::Error::new(
                                        segment.ident.span(),
                                        "distinct already exists",
                                    )
                                    .to_compile_error(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    std::result::Result::Ok(value)
}

fn get_settings_token_for_list(
    attributes: &Vec<String>,
    attribute_key: &str,
) -> proc_macro2::TokenStream {
    let string_attributes = attributes.iter().map(|attr| {
        quote! {
            #attr
        }
    });

    let ident = syn::Ident::new(attribute_key, proc_macro2::Span::call_site());

    if !attributes.is_empty() {
        quote! {
            .#ident([#(#string_attributes),*])
        }
    } else {
        proc_macro2::TokenStream::new()
    }
}

fn get_settings_token_for_string(
    attribute: &String,
    attribute_key: &str,
) -> proc_macro2::TokenStream {
    let ident = syn::Ident::new(attribute_key, proc_macro2::Span::call_site());

    if !attribute.is_empty() {
        quote! {
            .#ident(#attribute)
        }
    } else {
        proc_macro2::TokenStream::new()
    }
}
