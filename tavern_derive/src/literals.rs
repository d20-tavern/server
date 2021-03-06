use proc_macro2::{Span, TokenStream};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::LitStr;

pub(crate) fn lit_to_lit_str(l: &syn::Lit) -> Result<&LitStr, TokenStream> {
    match l {
        syn::Lit::Str(ls) => Ok(ls),
        _ => Err(compile_error_args!(l.span(), "expected a string literal")),
    }
}

pub(crate) fn lit_str_to_tokens(s: &LitStr) -> Result<TokenStream, TokenStream> {
    let tokens = syn::parse_str(&s.value())
        .map_err(|_| compile_error_args!("could not parse string as tokens"))?;

    Ok(tokens)
}

pub(crate) fn try_from_lit_str<T: Parse>(s: &LitStr) -> Result<T, TokenStream> {
    // serde_derive/src/internals/attr.rs:1931
    let tokens = lit_str_to_tokens(&s)?;
    let val: T = syn::parse2(tokens).map_err(|_| {
        compile_error_args!(
            Span::call_site(),
            "could not parse string {} as expected type {}",
            s.value(),
            std::any::type_name::<T>()
        )
    })?;
    Ok(val)
}
