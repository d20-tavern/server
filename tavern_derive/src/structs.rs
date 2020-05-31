use crate::literals;
use proc_macro2::{Span, TokenStream};
use std::convert::TryFrom;
use syn::spanned::Spanned;
use syn::{Attribute, Ident, LitStr, Meta, NestedMeta, Path};

pub(crate) struct DBStructAttrs {
    pub(crate) id_field: Ident,
    pub(crate) table: Option<Path>,
}

impl TryFrom<Vec<Attribute>> for DBStructAttrs {
    type Error = TokenStream;
    fn try_from(attrs: Vec<Attribute>) -> Result<Self, TokenStream> {
        let mut id_field = None;
        let mut table = None;

        for attr in attrs.iter() {
            match &attr.parse_meta().map_err(|err| err.to_compile_error())? {
                Meta::List(meta) => {
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
                                                        "id_field" => {
                                                            if id_field.is_some() {
                                                                return Err(compile_error_args!(meta.span(), "id_field inner attribute should only be set once"));
                                                            }
                                                            let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                            id_field = Some(literals::try_from_lit_str(&lit)?);
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
                Meta::NameValue(nv) => match nv.path.get_ident() {
                    None => {
                        return Err(compile_error_args!(
                            nv.span(),
                            "name_value attributes should not contain paths"
                        ))
                    }
                    Some(id) => match id.to_string().as_str() {
                        "table_name" => {
                            if table.is_some() {
                                return Err(compile_error_args!(
                                    nv.span(),
                                    "table_name attribute should only be set once"
                                ));
                            }
                            let lit = literals::lit_to_lit_str(&nv.lit)?;
                            table = Some(literals::try_from_lit_str(&lit)?);
                        }
                        _ => {
                            return Err(compile_error_args!(
                                nv.span(),
                                "unknown attribute name {}",
                                id
                            ))
                        }
                    },
                },
                Meta::Path(path) => {
                    return Err(compile_error_args!(
                        path.span(),
                        "unknown attribute {:#?}",
                        path
                    ))
                }
            }
        }

        Ok(DBStructAttrs {
            table,
            id_field: id_field.unwrap_or(literals::try_from_lit_str(&LitStr::new(
                "id",
                Span::call_site(),
            ))?),
        })
    }
}
