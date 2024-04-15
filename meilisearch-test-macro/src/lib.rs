#![recursion_limit = "4096"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Expr, FnArg, Ident, Item, PatType, Path, Stmt, Type, TypePath,
    Visibility,
};

#[proc_macro_attribute]
pub fn meilisearch_test(params: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        params.is_empty(),
        "the #[async_test] attribute currently does not take parameters"
    );

    let mut inner = parse_macro_input!(input as Item);
    let mut outer = inner.clone();
    if let (&mut Item::Fn(ref mut inner_fn), &mut Item::Fn(ref mut outer_fn)) =
        (&mut inner, &mut outer)
    {
        #[derive(Debug, PartialEq, Eq)]
        enum Param {
            Client,
            Index,
            String,
        }

        inner_fn.sig.ident = Ident::new(
            &("_inner_meilisearch_test_macro_".to_string() + &inner_fn.sig.ident.to_string()),
            Span::call_site(),
        );
        let inner_ident = &inner_fn.sig.ident;
        inner_fn.vis = Visibility::Inherited;
        inner_fn.attrs.clear();
        assert!(
            outer_fn.sig.asyncness.take().is_some(),
            "#[meilisearch_test] can only be applied to async functions"
        );

        let mut params = Vec::new();

        let parameters = &inner_fn.sig.inputs;
        for param in parameters {
            match param {
                FnArg::Typed(PatType { ty, .. }) => match &**ty {
                    Type::Path(TypePath { path: Path { segments, .. }, .. } ) if segments.last().unwrap().ident == "String" => {
                        params.push(Param::String);
                    }
                    Type::Path(TypePath { path: Path { segments, .. }, .. } ) if segments.last().unwrap().ident == "Index" => {
                        params.push(Param::Index);
                    }
                    Type::Path(TypePath { path: Path { segments, .. }, .. } ) if segments.last().unwrap().ident == "Client" => {
                        params.push(Param::Client);
                    }
                    // TODO: throw this error while pointing to the specific token
                    ty => panic!(
                        "#[meilisearch_test] can only receive Client, Index or String as parameters but received {ty:?}"
                    ),
                },
                // TODO: throw this error while pointing to the specific token
                // Used `self` as a parameter
                FnArg::Receiver(_) => panic!(
                    "#[meilisearch_test] can only receive Client, Index or String as parameters"
                ),
            }
        }

        // if a `Client` or an `Index` was asked for the test we must create a meilisearch `Client`.
        let use_client = params
            .iter()
            .any(|param| matches!(param, Param::Client | Param::Index));
        // if a `String` or an `Index` was asked then we need to extract the name of the test function.
        let use_name = params
            .iter()
            .any(|param| matches!(param, Param::String | Param::Index));
        let use_index = params.contains(&Param::Index);

        // Now we are going to build the body of the outer function
        let mut outer_block: Vec<Stmt> = Vec::new();

        // First we need to check if a client will be used and create it if it’s the case
        if use_client {
            outer_block.push(parse_quote!(
                let meilisearch_url = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
            ));
            outer_block.push(parse_quote!(
                let meilisearch_api_key = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
            ));
            outer_block.push(parse_quote!(
                let client = Client::new(meilisearch_url, Some(meilisearch_api_key)).unwrap();
            ));
        }

        // Now we do the same for the index name
        if use_name {
            let fn_name = &outer_fn.sig.ident;
            // the name we're going to return is the complete path to the function i.e., something like that;
            // `indexes::tests::test_fetch_info` but since the `::` are not allowed by meilisearch as an index
            // name we're going to rename that to `indexes-tests-test_fetch_info`.
            outer_block.push(parse_quote!(
                let name = format!("{}::{}", std::module_path!(), stringify!(#fn_name)).replace("::", "-");
            ));
        }

        // And finally if an index was asked, we delete it, and we (re)create it and wait until meilisearch confirm its creation.
        if use_index {
            outer_block.push(parse_quote!({
                let res = client
                    .delete_index(&name)
                    .await
                    .expect("Network issue while sending the delete index task")
                    .wait_for_completion(&client, None, None)
                    .await
                    .expect("Network issue while waiting for the index deletion");
                if res.is_failure() {
                    let error = res.unwrap_failure();
                    assert_eq!(
                        error.error_code,
                        crate::errors::ErrorCode::IndexNotFound,
                        "{:?}",
                        error
                    );
                }
            }));

            outer_block.push(parse_quote!(
                let index = client
                    .create_index(&name, None)
                    .await
                    .expect("Network issue while sending the create index task")
                    .wait_for_completion(&client, None, None)
                    .await
                    .expect("Network issue while waiting for the index creation")
                    .try_make_index(&client)
                    .expect("Could not create the index out of the create index task");
            ));
        }

        // Create a list of params separated by comma with the name we defined previously.
        let params: Vec<Expr> = params
            .into_iter()
            .map(|param| match param {
                Param::Client => parse_quote!(client),
                Param::Index => parse_quote!(index),
                Param::String => parse_quote!(name),
            })
            .collect();

        // Now we can call the user code with our parameters :tada:
        outer_block.push(parse_quote!(
            let result = #inner_ident(#(#params.clone()),*).await;
        ));

        // And right before the end, if an index was created and the tests successfully executed we delete it.
        if use_index {
            outer_block.push(parse_quote!(
                index
                    .delete()
                    .await
                    .expect("Network issue while sending the last delete index task");
                // we early exit the test here and let meilisearch handle the deletion asynchronously
            ));
        }

        // Finally, for the great finish we just return the result the user gave us.
        outer_block.push(parse_quote!(return result;));

        outer_fn.sig.inputs.clear();
        outer_fn.sig.asyncness = inner_fn.sig.asyncness;
        outer_fn.attrs.push(parse_quote!(#[tokio::test]));
        outer_fn.block.stmts = outer_block;
    } else {
        panic!("#[meilisearch_test] can only be applied to async functions")
    }
    quote!(
        #inner
        #outer
    )
    .into()
}
