extern crate proc_macro;

#[macro_use]
mod error;
mod literals;
mod structs;

use convert_case::{Case, Casing};
use core::convert::TryFrom;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::{Pair, Pairs};
use syn::token::Comma;
use syn::{Data, DeriveInput, Field, Fields};

use crate::structs::DBStructAttrs;

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
            let conn = crate::db::get_connection().await
                .expect("database reset failed");
            let mut tx = conn.begin().await
                .map_err(|err| crate::db::Error::Transaction(err))
                .expect("database reset failed");
            sqlx::query(r"DROP SCHEMA IF EXISTS tavern CASCADE")
                .execute(&mut tx)
                .await
                .map_err(|err| crate::db::Error::RunQuery(err))
                .expect("database reset failed");
            tx.commit().await
                .map_err(|err| crate::db::Error::Transaction(err))
                .expect("database reset failed");

            crate::db::init()
                .await
                .expect("database initialization failed");

            #body
        }
    };

    result.into()
}

fn field_from_iter<'a, 'b>(iter: Pairs<'a, Field, Comma>, name: &'b str) -> Option<&'a syn::Ident> {
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
    let fields;

    match &input.data {
        Data::Struct(struc) => fields = &struc.fields,
        _ => {
            return compile_error_args!(
                input.ident.span(),
                "this derive only applies to structs with named fields"
            )
            .into()
        }
    };

    let result = if let Fields::Named(named) = fields {
        let id = field_from_iter(named.named.pairs(), "id").unwrap_or_else(|| {
            panic!(
                "auto creation of Summary type for {} requires field 'id'",
                name
            )
        });
        let fname = field_from_iter(named.named.pairs(), "name").unwrap_or_else(|| {
            panic!(
                "auto creation of Summary type for {} requires field 'name'",
                name
            )
        });
        let desc = field_from_iter(named.named.pairs(), "description")
                    .unwrap_or_else(|| panic!("auto creation of Summary type for {} requires field 'short_description' or 'description'", name));
        let links = field_from_iter(named.named.pairs(), "links")
            .map(|id| quote! { Some(&self.#id) })
            .unwrap_or_else(|| quote! { None });

        quote! {
            impl crate::pathfinder::summary::Summarize<#name> for #name {
                fn id(&self) -> &uuid::Uuid {
                    &self.#id
                }

                fn name(&self) -> &str {
                    &self.#fname
                }

                fn description(&self) -> &str {
                    &self.#desc
                }

                fn links(&self) -> Option<&crate::pathfinder::Links> {
                    #links
                }
            }
        }
    } else {
        compile_error_args!(
            input.ident.span(),
            "can only impl_summary on structs with named fields"
        )
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
        return compile_error_args!(
            input.ident.span(),
            "this macro only works on enums with all unit variants"
        )
        .into();
    };

    for v in input.variants.pairs() {
        match v.value().fields {
            Fields::Unit => {}
            _ => {
                return compile_error_args!(
                    v.value().ident.span(),
                    "this macro only works on enums with all unit variants"
                )
                .into()
            }
        }
    }

    let var_words = input
        .variants
        .pairs()
        .map(|v| v.value().ident.to_string().to_case(Case::Lower));

    let var = input.variants.pairs().zip(var_words).map(|(v, s)| {
        let v = &v.value().ident;
        quote! { #name::#v => #s }
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

#[proc_macro_derive(GetById, attributes(table_name, tavern))]
pub fn derive_get_by_id(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = match DBStructAttrs::try_from(input.attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into(),
    };
    let table = attrs.table.ok_or_else(|| {
        compile_error_args!(
            name.span(),
            "tavern(table) attribute expect to map to object under crate::db::schemas"
        )
    });
    let table = match table {
        Ok(t) => t,
        Err(err) => return err.into(),
    };
    let id_field = attrs.id_field;

    let result = quote! {
        impl crate::db::GetById for #name {
            fn db_get_by_id(by_id: &uuid::Uuid, conn: &crate::db::Connection) -> Result<Self, crate::db::Error> {
                use crate::schema::#table::dsl::*;
                use crate::diesel::ExpressionMethods;
                use crate::diesel::RunQueryDsl;
                use crate::diesel::QueryDsl;
                #table.filter(#id_field.eq(by_id))
                    .first::<#name>(conn)
                    .map_err(crate::db::Error::RunQuery)
            }
        }
    };

    result.into()
}

#[proc_macro_derive(GetAll, attributes(table_name, tavern))]
pub fn derive_get_all(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = match DBStructAttrs::try_from(input.attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into(),
    };
    let table = attrs.table.ok_or_else(|| {
        compile_error_args!(
            name.span(),
            "tavern(table) attribute expected, maps to object under crate::db::schemas"
        )
    });
    let table = match table {
        Ok(t) => t,
        Err(err) => return err.into(),
    };

    let result = quote! {
        impl crate::db::GetAll for #name {
            fn db_get_all(conn: &crate::db::Connection) -> Result<Vec<Self>, crate::db::Error> {
                use crate::schema::#table::dsl::*;
                use crate::diesel::ExpressionMethods;
                use crate::diesel::RunQueryDsl;
                use crate::diesel::QueryDsl;
                #table
                    .load::<#name>(conn)
                    .map_err(crate::db::Error::RunQuery)
            }
        }
    };

    result.into()
}

#[proc_macro_derive(Insert, attributes(table_name, tavern))]
pub fn derive_insert(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = match DBStructAttrs::try_from(input.attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into(),
    };
    let table = attrs.table.ok_or_else(|| {
        compile_error_args!(
            name.span(),
            "tavern(table) attribute expect to map to object under crate::db::schemas"
        )
    });
    let table = match table {
        Ok(t) => t,
        Err(err) => return err.into(),
    };

    let result = quote! {
        impl crate::db::Insert for #name {
            fn db_insert(&self, conn: &crate::db::Connection) -> Result<(), crate::db::Error> {
                use crate::schema::#table::dsl::*;
                use crate::diesel::ExpressionMethods;
                use crate::diesel::RunQueryDsl;
                use crate::diesel::QueryDsl;
                diesel::insert_into(#table)
                    .values(self)
                    .execute(conn)
                    .map_err(crate::db::Error::RunQuery)
                    .map(|_| ())
            }
        }
    };

    result.into()
}

#[proc_macro_derive(Update, attributes(table_name, tavern))]
pub fn derive_update(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = match DBStructAttrs::try_from(input.attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into(),
    };
    let table = attrs.table.ok_or_else(|| {
        compile_error_args!(
            name.span(),
            "tavern(table) attribute expect to map to object under crate::db::schemas"
        )
    });
    let table = match table {
        Ok(t) => t,
        Err(err) => return err.into(),
    };
    let id_field = attrs.id_field;

    let result = quote! {
        impl crate::db::Update for #name {
            fn db_update(&self, conn: &crate::db::Connection) -> Result<(), crate::db::Error> {
                use crate::schema::#table::dsl::*;
                use crate::diesel::ExpressionMethods;
                use crate::diesel::RunQueryDsl;
                use crate::diesel::QueryDsl;
                diesel::update(#table.filter(#id_field.eq(&self.id)))
                    .set(self)
                    .execute(conn)
                    .map_err(crate::db::Error::RunQuery)
                    .map(|_| ())
            }
        }
    };

    result.into()
}

#[proc_macro_derive(Delete, attributes(table_name, tavern))]
pub fn derive_delete(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = match DBStructAttrs::try_from(input.attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into(),
    };
    let table = attrs.table.ok_or_else(|| {
        compile_error_args!(
            name.span(),
            "tavern(table) attribute expect to map to object under crate::db::schemas"
        )
    });
    let table = match table {
        Ok(t) => t,
        Err(err) => return err.into(),
    };
    let id_field = attrs.id_field;

    let result = quote! {
        impl crate::db::Delete for #name {
            fn db_delete(&self, conn: &crate::db::Connection) -> Result<(), crate::db::Error> {
                use crate::schema::#table::dsl::*;
                use crate::diesel::ExpressionMethods;
                use crate::diesel::RunQueryDsl;
                use crate::diesel::QueryDsl;
                diesel::delete(#table.filter(#id_field.eq(&self.#id_field)))
                    .execute(conn)
                    .map_err(crate::db::Error::RunQuery)
                    .map(|_| ())
            }
        }
    };

    result.into()
}
