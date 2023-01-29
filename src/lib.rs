extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{Expr, Lit};

/// Convenient way of making a `String`.
///
/// 1. `s!()`
/// 2. `s!(123 + 321)`
/// 3. `s!("hello, {}", "world")`
///
/// # Examples
///
/// ```
/// use s_macro::s;
///
/// assert!(s!()                   == String::new());
/// assert!(s!("hello, world")     == String::from("hello, world"));
/// assert!(s!(123 + 321)          == format!("{}", 123 + 321));
///
/// let world = "world";
/// assert!(s!("hello, {}", world) == format!("hello, {}", world));
/// assert!(s!("hello, {world}")   == format!("hello, {world}"));
/// ```
#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        quote! { String::new() }.into()
    }

    else if let Ok(expr) = syn::parse::<Expr>(input.clone()) {
        if let Expr::Lit(ref lit) = expr {
            if let Lit::Str(_) = lit.lit {
                return quote! { format!(#expr) }.into();
            }
        }

        quote! { format!("{}", #expr) }.into()
    }

    else {
        let input = proc_macro2::TokenStream::from(input);
        quote! { format!(#input) }.into()
    }
}
