extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::punctuated::{Pair, Pairs};
use syn::{token::Comma, Data, Field, Fields, Ident};
use convert_case::{Case, Casing};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[proc_macro_attribute]
pub fn db_test(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let asyncness = &input.sig.asyncness;
    let vis = input.vis;

    let result = quote! {
        #[tokio::test]
        #[cfg(feature = "db-test")]
        #(#attrs)*
        #asyncness #vis fn #name() #ret {
            use sqlx::Connection;
            let conn = tavern_server::db::get_connection().await
                .expect("database reset failed");
            let mut tx = conn.begin().await
                .map_err(|err| tavern_server::db::Error::Transaction(err))
                .expect("database reset failed");
            sqlx::query(r"DROP SCHEMA IF EXISTS tavern CASCADE")
                .execute(&mut tx)
                .await
                .map_err(|err| tavern_server::db::Error::RunQuery(err))
                .expect("database reset failed");
            tx.commit().await
                .map_err(|err| tavern_server::db::Error::Transaction(err))
                .expect("database reset failed");

            tavern_server::db::init()
                .await
                .expect("database initialization failed");

            #body
        }
    };

    result.into()
}

fn field_from_iter<'a, 'b>(iter: Pairs<'a, Field, Comma>, name: &'b str) -> Option<&'a Ident> {
    let iter = iter.filter_map(|item| match item {
        Pair::Punctuated(field, _) => Some(field),
        _ => None,
    });
    iter.clone()
        .find(|item| {
            item.attrs
                .iter()
                .any(|attr| attr.path.get_ident().map(|id| id == name).unwrap_or(false))
        })
        .or_else(|| {
            iter.clone().find(|item| {
                if let Some(id) = &item.ident {
                    id == name
                } else {
                    false
                }
            })
        })
        .map(|field| field.ident.as_ref().unwrap())
}

#[proc_macro_derive(Summarize, attributes(id, name, description, links))]
pub fn derive_summarize(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let name = &input.ident;
    let mut fields;

    match &input.data {
        Data::Struct(struc) => fields = &struc.fields,
        _ => {
            return quote_spanned! {
                input.ident.span() => 
                    compile_error!("this derive only applies to structs with named fields")
            }.into();
        },
    };

    let result = if let Fields::Named(named) = fields {
        let id = field_from_iter(named.named.pairs(), "id").expect(&format!(
            "auto creation of Summary type for {} requires field 'id'",
            name
        ));
        let fname = field_from_iter(named.named.pairs(), "name").expect(&format!(
            "auto creation of Summary type for {} requires field 'name'",
            name
        ));
        let desc = field_from_iter(named.named.pairs(), "description")
                    .expect(&format!("auto creation of Summary type for {} requires field 'short_description' or 'description'", name));
        let links = field_from_iter(named.named.pairs(), "links").expect(&format!(
            "auto creation of Summary type for {} requires field 'links'",
            name
        ));

        quote! {
            impl crate::summary::Summarize<#name> for #name {
                fn id(&self) -> &uuid::Uuid {
                    &self.#id
                }

                fn name(&self) -> &str {
                    &self.#fname
                }

                fn description(&self) -> &str {
                    &self.#desc
                }

                fn links(&self) -> &crate::Links {
                    &self.#links
                }
            }
        }
    } else {
        quote_spanned! {
            input.ident.span() =>
                compile_error!("can only impl_summary on structs with named fields")
        }
    };

    result.into()
}

#[proc_macro_derive(Display)]
pub fn impl_enum_display(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let input = if let Data::Enum(e) = &input.data {
        e
    } else {
        return quote_spanned! {
            input.ident.span() =>
            ("this macro only works on enums with all unit variants")
        }.into();
    };

    for v in input.variants.pairs() {
        let v = v.value();
        if v.fields != Fields::Unit {
            return quote_spanned! {
                v.ident.span() =>
                    compile_error!("this macro only works on enums with all unit variants")
            }.into();
        }
    }

    let var_words = input.variants.pairs()
        .map(|v| v.value().ident.to_string().to_case(Case::Lower));

    let var = input.variants.pairs().zip(var_words)
        .map(|(v, s)| {
            let v = &v.value().ident;
            quote!{ #name::#v => #s }
        });
    
    let result = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let val = match self {
                    #(#var,)*
                };
                write!(f, "{}", val)
            }
        }
    };

    result.into()
}
