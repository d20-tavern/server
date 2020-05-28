use std::convert::TryFrom;
use std::fmt::Debug;
use syn::{Attribute, Meta, NestedMeta};
use syn::spanned::Spanned;
use quote::quote;

use crate::literals;

pub(crate) struct DBStructAttrs {
    pub(crate) id_column: String,
    pub(crate) select_post_op: Option<proc_macro2::TokenStream>,
    pub(crate) table_name: Option<String>,
    pub(crate) verify_user: Option<String>,
}

impl TryFrom<Vec<Attribute>> for DBStructAttrs {
    type Error = proc_macro2::TokenStream;
    fn try_from(attrs: Vec<Attribute>) -> Result<Self, proc_macro2::TokenStream> {
        let mut id_column = None;
        let mut select_post_op = None;
        let mut table_name = None;
        let mut verify_user = None;

        for attr in attrs.iter() {
            if let Meta::List(meta) = &attr.parse_meta().map_err(|err| err.to_compile_error())? {
                if let Some(meta_id) = &meta.path.get_ident() {
                    if *meta_id == "tavern" {
                        for nested in meta.nested.pairs() {
                            match &nested.value() {
                                NestedMeta::Meta(inner) => {
                                    match inner {
                                        Meta::NameValue(nv) => match nv.path.get_ident() {
                                            None => return Err(compile_error_args!(meta.span(), "tavern inner attributes should not contain paths")),
                                            Some(id) => {
                                                match id.to_string().as_str() {
                                                    "id_column" => {
                                                        if id_column.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "post_op inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        id_column = Some(lit.value());
                                                    },
                                                    "select_post_op" => {
                                                        if select_post_op.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "post_op inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        select_post_op = Some(literals::lit_str_to_tokens(&lit)?);
                                                    },
                                                    "table_name" => {
                                                        if table_name.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "post_op inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        table_name = Some(lit.value());
                                                    },
                                                    "verify_user" => {
                                                        if verify_user.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "verify_user inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        verify_user = Some(lit.value());
                                                    },
                                                    _ => return Err(compile_error_args!(meta.span(), "unknown attribute name {}", id)),
                                                }
                                            }
                                        },
                                        _ => return Err(compile_error_args!(meta.span(), "tavern attribute currently only takes name-value pairs")),
                                    }
                                },
                                NestedMeta::Lit(lit) => {
                                    return Err(compile_error_args!(meta.span(), "unknown tavern attribute {:?}", lit));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(DBStructAttrs{ select_post_op, table_name, verify_user, id_column: id_column.unwrap_or("id".to_string()) })
    }
}

impl DBStructAttrs {
    pub(crate) fn select_post_op(&self) -> proc_macro2::TokenStream {
        match &self.select_post_op {
            None => quote!{},
            Some(tok) => tok.to_owned(),
        }
    }
}