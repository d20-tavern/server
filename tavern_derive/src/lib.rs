extern crate proc_macro;

#[macro_use]
mod error;
mod fields;
mod literals;
mod structs;

use core::convert::TryFrom;
use proc_macro::TokenStream;
use proc_macro2;
use quote::{format_ident, quote};
use syn::punctuated::{Pair, Pairs};
use syn::token::Comma;
use syn::{Data, DeriveInput, Field, Fields};
use convert_case::{Case, Casing};

use crate::fields::DBField;
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
            let conn = tavern_db::get_connection().await
                .expect("database reset failed");
            let mut tx = conn.begin().await
                .map_err(|err| tavern_db::Error::Transaction(err))
                .expect("database reset failed");
            sqlx::query(r"DROP SCHEMA IF EXISTS tavern CASCADE")
                .execute(&mut tx)
                .await
                .map_err(|err| tavern_db::Error::RunQuery(err))
                .expect("database reset failed");
            tx.commit().await
                .map_err(|err| tavern_db::Error::Transaction(err))
                .expect("database reset failed");

            tavern_db::init()
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
        _ => return compile_error_args!(input.ident.span(), "this derive only applies to structs with named fields").into(),
    };

    let result = if let Fields::Named(named) = fields {
        let id = field_from_iter(named.named.pairs(), "id")
            .unwrap_or_else(|| panic!(
            "auto creation of Summary type for {} requires field 'id'",
            name
        ));
        let fname = field_from_iter(named.named.pairs(), "name").unwrap_or_else(|| panic!(
            "auto creation of Summary type for {} requires field 'name'",
            name
        ));
        let desc = field_from_iter(named.named.pairs(), "description")
                    .unwrap_or_else(|| panic!("auto creation of Summary type for {} requires field 'short_description' or 'description'", name));
        let links = field_from_iter(named.named.pairs(), "links").unwrap_or_else(|| panic!(
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
        compile_error_args!(input.ident.span(), "can only impl_summary on structs with named fields")
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
        return compile_error_args!(input.ident.span(), "this macro only works on enums with all unit variants").into();
    };

    for v in input.variants.pairs() {
        match v.value().fields {
            Fields::Unit => {},
            _ => return compile_error_args!(v.value().ident.span(), "this macro only works on enums with all unit variants").into(),
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

#[proc_macro_derive(TryFromRow, attributes(tavern))]
pub fn derive_try_from_row(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);

    // Get fields
    // For each field
    // - Get column name/command (i.e. field name or column attr, if given)
    // - Get type (has references = Uuid)
    // - If references, call try_from_uuid on the reference type

    let name = &input.ident;
    let struct_attrs = match DBStructAttrs::try_from(input.attrs.clone()) {
        Ok(sa) => sa,
        Err(err) => return err.into(),
    };
    let post_op = struct_attrs.select_post_op();
    let struc = if let Data::Struct(struc) = &input.data {
        syn::DataStruct::to_owned(struc)
    } else {
        return compile_error_args!(input.ident.span(), "TryFromRow can only be derived on structs with named parameters").into();
    };

    let fields = if let Fields::Named(named) = &struc.fields {
        named.named.to_owned()
    } else {
        return compile_error_args!(input.ident.span(), "TryFromRow can only be derived on structs with named parameters").into();
    };

    let get_fields: Result<Vec<_>, _> = fields.iter()
        .map(|f| {
            let f: Field = Field::to_owned(f);
            DBField::try_from(f)
        })
        .map(|dbf| {
            Ok(dbf?.impl_try_from_row()?)
        })
        .collect();

    let get_fields = match get_fields {
        Ok(get_fields) => get_fields,
        Err(e) => return e,
    };

    let field_names = fields.iter()
        .map(|field| {
            field.ident.as_ref().unwrap()
        });

    let result = quote! {
        #[async_trait::async_trait]
        impl tavern_db::TryFromRow for #name {
            async fn try_from_row<'a>(row: &'a sqlx::postgres::PgRow<'_>, user: &'a uuid::Uuid) -> Result<Self, tavern_db::Error> {
                use sqlx::row::Row;
                use tavern_db::TryFromUuid;
                use tokio::stream::Stream as TokioStream;
                use futures::stream::Stream;
                //use tokio::stream::StreamExt as TokioStreamExt;
                use futures::stream::StreamExt;

                #(#get_fields)*
                
                let mut instance = #name {
                    #(#field_names),*
                };

                #post_op

                Ok(instance)
            }
        }
    };

    result.into()
}

#[proc_macro_derive(TryFromUuid, attributes(tavern))]
pub fn derive_try_from_uuid(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);

    // Get fields
    // For each field
    // - Get column selection, incl. column name if necessary -- could just always do "AS ___"
    // - Allow specifying table name
    // - Tag id field for attaching id to
    // - Always LIMIT 1
    // - Fetch
    // - Get first row
    // - If verify_user, get user and compare id or is_admin
    //   - If invalid, return not authorized
    //   - If valid, do try_from_row
    // - Commit

    let name = &input.ident;
    let struct_attrs = DBStructAttrs::try_from(input.attrs.clone())
        .map_err(proc_macro2::TokenStream::into);

    let struct_attrs = match struct_attrs {
        Ok(s) => s,
        Err(e) => return e,
    };

    let struc = if let Data::Struct(struc) = &input.data {
        syn::DataStruct::to_owned(struc)
    } else {
        return compile_error_args!("TryFromUuid can only be derived on structs with named parameters").into();
    };

    let fields = if let Fields::Named(named) = &struc.fields {
        named.named.to_owned()
    } else {
        return compile_error_args!("TryFromUuid can only be derived on structs with named parameters").into();
    };

    let select_fields: Result<Vec<_>, proc_macro2::TokenStream> = fields.iter()
        .map(Field::to_owned)
        .map(DBField::try_from)
        .collect();

    let select_fields = match select_fields {
        Err(e) => return e.into(),
        Ok(fields) => {
            fields.iter().filter(|dbf| !dbf.skip)
                .map(|dbf| {
                    format!("{} AS {}", dbf.column(), dbf.get_column_name())
                })
                .collect::<Vec<String>>()
                .join(", ")
        }
    };

    let user_var = if struct_attrs.verify_user.is_some() {
        format_ident!("user")
    } else {
        format_ident!("_user")
    };

    let query = {
        let table = struct_attrs.table_name.unwrap_or(name.to_string());
        let query_str = format!(r"SELECT {} FROM {} WHERE {}.{} = $1 LIMIT 1",
            select_fields, table, table, struct_attrs.id_column);
        let user_bind = if struct_attrs.verify_user.is_some() {
            quote!{ .bind(&#user_var); }
        } else {
            quote!{ ; }
        };
        quote! {
            let query = sqlx::query(#query_str)
                .bind(&id)#user_bind
        }
    };

    let verify_user = match struct_attrs.verify_user {
        None => {
            quote!{}
        },
        Some(uid) => {
            // TODO: Allow admins too
            quote!{
                let user_id: uuid::Uuid = row.try_get(#uid)
                    .map_err(tavern_db::Error::RunQuery)?;

                if uuid::Uuid::to_owned(#user_var) != user_id {
                    return Err(tavern_db::Error::UserUnauthorized(uuid::Uuid::to_owned(#user_var)))
                }
            }
        },
    };

    let result = quote! {
        #[async_trait::async_trait]
        impl tavern_db::TryFromUuid for #name {
            async fn try_from_uuid(id: uuid::Uuid, #user_var: &uuid::Uuid) -> Result<Self, tavern_db::Error> {
                use sqlx::row::Row;
                use sqlx::{Connection, Cursor};
                use tavern_db::TryFromRow;

                let conn: tavern_db::Connection = tavern_db::get_connection().await?;

                let mut tx = conn.begin().await
                    .map_err(tavern_db::Error::Connection)?;

                #query

                let mut rows = query.fetch(&mut tx);

                let row = rows.next().await
                    .map_err(tavern_db::Error::RunQuery)?
                    .ok_or_else(|| tavern_db::Error::NoRows)?;

                #verify_user

                let instance = #name::try_from_row(&row, &#user_var).await?;

                tx.commit()
                    .await
                    .map_err(tavern_db::Error::Transaction)?;

                Ok(instance)
            }
        }
    };

    result.into()
}
