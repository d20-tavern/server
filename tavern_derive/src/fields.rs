use proc_macro2;
use quote::quote;
use syn::{NestedMeta, Field, Meta};
use syn::spanned::Spanned;
use std::convert::TryFrom;
use std::fmt::Debug;
use crate::literals;

pub(crate) struct DBField {
    pub(crate) field: syn::Field,
    pub(crate) ident: syn::Ident,
    pub(crate) skip: bool,
    pub(crate) is_array: bool,
    pub(crate) is_map: bool,
    pub(crate) is_optional: bool,
    pub(crate) is_set: bool,
    pub(crate) tuple_hack: Option<syn::Type>,
    pub(crate) default: Option<syn::Expr>,
    pub(crate) column: Option<String>,
    pub(crate) column_name: Option<String>,
    pub(crate) references: Option<syn::Type>,
    pub(crate) key_references: Option<syn::Type>,
    pub(crate) key_type: Option<syn::Type>,
    pub(crate) val_references: Option<syn::Type>,
    pub(crate) val_type: Option<syn::Type>,
}

impl TryFrom<Field> for DBField {
    type Error = proc_macro2::TokenStream;
    fn try_from(field: Field) -> Result<DBField, proc_macro2::TokenStream> {
        let mut column = None;
        let mut column_name = None;
        let mut default = None;
        let mut references = None;
        let mut key_type = None;
        let mut key_references = None;
        let mut val_type = None;
        let mut val_references = None;
        let mut tuple_hack = None;
        let mut skip = false;
        let mut is_array = false;
        let mut is_map = false;
        let mut is_optional = false;
        let mut is_set = false;
        let ident = match &field.ident {
            Some(id) => id.to_owned() ,
            None => return Err(compile_error_args!(field.span(), "struct field should have an identifier")),
        };

        for attr in &field.attrs {
            if let Meta::List(meta) = &attr.parse_meta().map_err(|err| err.to_compile_error())? {
                if let Some(meta_id) = &meta.path.get_ident() {
                    if *meta_id == "tavern" {
                        for nested in meta.nested.pairs() {
                            match &nested.value() {
                                NestedMeta::Meta(inner) => {
                                    match inner {
                                        Meta::Path(path) => {
                                            match path.get_ident() {
                                                None => return Err(compile_error_args!(meta.span(), "tavern inner attributes should not contain paths")),
                                                Some(id) => {
                                                    match id.to_string().as_str() {
                                                        "skip" => skip = true,
                                                        "is_array" => is_array = true,
                                                        "is_map" => is_map = true,
                                                        "is_optional" => is_optional = true,
                                                        "is_set" => is_set = true,
                                                        _ => return Err(compile_error_args!(meta.span(), "unknown attribute name {}", id)),
                                                    }
                                                }
                                            }
                                        },
                                        Meta::NameValue(nv) => match nv.path.get_ident() {
                                            None => return Err(compile_error_args!(meta.span(), "tavern inner attributes should not contain paths")),
                                            Some(id) => {
                                                match id.to_string().as_str() {
                                                    "column" => {
                                                        if column.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "column inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        column = Some(lit.value());
                                                    },
                                                    "column_name" => {
                                                        if column_name.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "column_name inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        column_name = Some(lit.value());
                                                    },
                                                    "default" => {
                                                        if default.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "default inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        default = Some(literals::try_from_lit_str(lit)?);
                                                    },
                                                    "key_references" => {
                                                        if key_references.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "key_references inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        key_references = Some(literals::try_from_lit_str(lit).
                                                            map_err(|_| compile_error_args!("could not parse key_references string as Type"))?);
                                                    },
                                                    "key_type" => {
                                                        if key_type.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "key_type inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        key_type = Some(literals::try_from_lit_str(lit).
                                                            map_err(|_| compile_error_args!("could not parse key_type string as Type"))?);
                                                    },
                                                    "val_references" => {
                                                        if val_references.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "val_references inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        val_references = Some(literals::try_from_lit_str(lit).
                                                            map_err(|_| compile_error_args!("could not parse val_references string as Type"))?);
                                                    },
                                                    "val_type" => {
                                                        if val_type.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "val_type inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        val_type = Some(literals::try_from_lit_str(lit).
                                                            map_err(|_| compile_error_args!("could not parse val_type string as Type"))?);
                                                    },
                                                    "references" => {
                                                        if references.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "references inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        references = Some(literals::try_from_lit_str(lit).
                                                            map_err(|_| compile_error_args!("could not parse references string as Type"))?);
                                                    },
                                                    "tuple_hack" => {
                                                        if tuple_hack.is_some() {
                                                            return Err(compile_error_args!(meta.span(), "tuple_hack inner attribute should only be set once"));
                                                        }
                                                        let lit = literals::lit_to_lit_str(&nv.lit)?;
                                                        tuple_hack = Some(literals::try_from_lit_str(lit).
                                                            map_err(|_| compile_error_args!("could not parse tuple_hack string as Type"))?);
                                                    },
                                                    _ => return Err(compile_error_args!(meta.span(), "unknown tavern attribute {}", id)),
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

        Ok(DBField{ column_name, field, ident, column, references, default, skip,
            is_array, is_map, is_optional, is_set, tuple_hack,
            key_type, key_references, val_type, val_references })
    }
}

impl DBField {
    pub(crate) fn column(&self) -> String {
        match &self.column {
            Some(sel) => sel.to_owned(),
            None => match &self.column_name {
                Some(name) => name.to_owned(),
                None => {
                    let id = &self.ident;
                    id.to_string()
                },
            },
        }
    }

    pub(crate) fn get_column_name(&self) -> String {
        match &self.column_name {
            Some(name) => name.to_owned(),
            None => {
                let id = &self.ident;
                id.to_string()
            },
        }
    }

    pub(crate) fn typ(&self) -> &syn::Type {
        &self.field.ty
    }

    fn try_from_row_skip(&self) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
        if self.skip {
            let name = &self.ident;
            let typ = self.typ();

            match self.default.as_ref() {
                None => Err(compile_error_args!(self.ident.span(), "skip must be used with a default value")),
                Some(expr) => Ok(quote! {
                    let #name: #typ = #expr;
                }),
            }
        } else {
            Err(compile_error_args!(self.ident.span(),
                "internal: try_from_row_skip called on field not marked as skip"))
        }
    }

    fn try_from_row_column_type(&self) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
        match self.references.as_ref() {
            Some(_) => {
                let typ = if self.is_array || self.is_set {
                    quote! { Vec<uuid::Uuid> }
                } else if self.is_map {
                    let infer = syn::parse_str::<syn::token::Underscore>("_")
                        .map_err(|_| compile_error_args!(
                                "internal: could not parse underscore as type inference"
                            )
                        )?;
                    let key = match self.key_references.as_ref() {
                        Some(_) => literals::type_from_path_str("uuid::Uuid")
                            .map_err(|_| compile_error_args!(
                                    "internal: failed to parse uuid::Uuid as type path"
                                )
                            )?,
                        None => match self.key_type.as_ref() {
                            Some(typ) => syn::Type::to_owned(typ),
                            None => syn::Type::Infer(syn::TypeInfer{
                                underscore_token: infer.to_owned(),
                            }),
                        }
                    };

                    let val = match self.val_references.as_ref() {
                        Some(_) => literals::type_from_path_str("uuid::Uuid")
                            .map_err(|_|
                                compile_error_args!(
                                    "internal: failed to parse uuid::Uuid as type path"
                                )
                            )?,
                        None => match self.val_type.as_ref() {
                            Some(typ) => syn::Type::to_owned(typ),
                            None => syn::Type::Infer(syn::TypeInfer{
                                underscore_token: infer.to_owned(),
                            }),
                        }
                    };
                    quote!{ Vec<(#key, #val)> }
                } else {
                    quote!{ uuid::Uuid }
                };

                let typ = if self.is_optional {
                    Ok(quote!{ Option<#typ> })
                } else {
                    Ok(typ)
                };

                typ
            },
            None => {
                match self.tuple_hack.as_ref() {
                    None => {
                        let typ = self.typ();
                        Ok(quote! { #typ })
                    },
                    Some(typ) => {
                        if self.is_array || self.is_set {
                            Ok(quote!{ Vec<(#typ,)> })
                        } else {
                            // Workaround shouldn't need to apply to other cases
                            Err(compile_error_args!(
                                self.ident.span(),
                                "field {} does not need the tuple hack",
                                self.ident))
                        }
                    }
                }
            },
        }
    }

    fn try_from_row_column(&self) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
        let name = &self.ident;
        let typ = self.try_from_row_column_type()?;
        let column_name = self.get_column_name();
        let get_column = match self.default.as_ref() {
            None => {
                quote! {
                    let #name: #typ = row.try_get(#column_name)
                        .map_err(tavern_db::Error::RunQuery)?;
                }
            },
            Some(def) => {
                quote! {
                    let #name: #typ = match row.try_get(#column_name) {
                        Ok(val) => val,
                        Err(sqlx::Error::ColumnNotFound(_)) => #def,
                        Err(err) => return Err(tavern_db::Error::RunQuery(err)),
                    };
                }
            }
        };
        Ok(get_column)
    }

    fn try_from_row_transform_column(&self) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
        let name = &self.ident;
        let get_ref = match self.references.as_ref() {
            None => match self.tuple_hack.as_ref() {
                // TODO: Enforce is_collections
                None => quote!{},
                Some(_typ) => {
                    let final_type = self.typ();
                    quote! {
                        let #name: #final_type = #name.iter()
                            .map(|(item,)| *item)
                            .collect();
                    }
                }
            },
            Some(t) => {
                let final_type = self.typ();
                let get_ref = if self.is_array || self.is_set {
                    quote! {
                        let #name: Vec<Result<_, _>> = #name.iter()
                            .map(|id| <#t>::try_from_uuid(id, user))
                            .collect::<futures::stream::FuturesUnordered<_>>()
                            .collect()
                            .await;
                        let #name: #final_type = #name.iter().collect()?;
                    }
                } else if self.is_map {
                    let key_val_map = if self.key_references.is_some() || self.val_references.is_some() {
                        let key_map = match &self.key_references {
                            None => quote!{},
                            Some(t) => quote! {
                                let key = <#t>::try_from_uuid(key, user).await?;
                            }
                        };
                        let val_map = match &self.val_references {
                            None => quote!{},
                            Some(t) => quote! {
                                let val = <#t>::try_from_uuid(val, user).await?;
                            }
                        };
                        quote! {
                            .map(|(key, val)| async {
                                #key_map
                                #val_map
                                Ok((key, val))
                            })
                            .collect::<futures::stream::FuturesUnordered<_>>()
                        }
                    } else {
                        quote!{.map(|(key, val)| Ok((key, val)) )}
                    };

                    quote! {
                        //let #name: #typ = #name.iter()
                        let #name: Vec<Result<_,_>> = #name.iter()
                            #key_val_map
                            .collect::<Result<#final_type, tavern_db::Error>>()
                            .await;
                        let #name: #final_type = #name.iter().collect()?;
                    }
                } else {
                    quote! {
                        //let #name: #typ = <#t>::try_from_uuid(#name, &user)?;
                        let #name: #final_type = <#t>::try_from_uuid(#name, user).await?;
                    }
                };

                let get_ref = if self.is_optional {
                    quote! {
                        let #name = #name.map(|#name| async {
                            #get_ref
                            Ok(#name)
                        })
                        .into::<futures::future::OptionFuture>()
                        .await
                        .transpose()?;
                    }
                } else {
                    get_ref
                };

                get_ref
            },
        };

        Ok(get_ref)
    }

    pub(crate) fn impl_try_from_row(&self) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
        if self.skip {
            self.try_from_row_skip()
        } else {
            let get_field = self.try_from_row_column()?;
            let get_ref = self.try_from_row_transform_column()?;

            let result = quote!{
                #get_field
                #get_ref
            };

            Ok(result)
        }
    }
}