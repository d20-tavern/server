
// TODO: For Rust 1.45, replace call_site with mixed_site
macro_rules! compile_error_args {
    ($msg:expr) => {
        compile_error_args!(proc_macro2::Span::call_site(), $msg)
    };
    ($span:expr, $msg:expr) => {
        quote::quote_spanned! {
            $span=>
            compile_error!($msg);
        }
    };
    ($span:expr, $msg:expr,) => {
        quote::quote_spanned! {
            $span=>
            compile_error!($msg);
        }
    };
    ($span:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        quote::quote_spanned! {
            $span=>
                compile_error!(#msg);
        }
    }};
}
