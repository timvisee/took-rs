extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, parse_quote, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

/// Measure run time of a function and report elapsed time.
///
/// This macro can be attached to a function to measure it's running time.
/// The run time is printed to the console (`stderr`) in human readable format when the function
/// returns.
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate took_macro;
///
/// #[took(description = "A heavy task")]
/// fn my_function() {
///     // Written to stderr on return:
///     // A heavy task took 1.00 s
/// }
///
/// #[took]
/// fn other_function() {
///     // Written to stderr on return:
///     // other_function() took 1.00 s
/// }
/// ```
#[proc_macro_attribute]
pub fn took(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments, and item as function
    let args: Vec<NestedMeta> = parse_macro_input!(attr as AttributeArgs);
    let mut item = parse_macro_input!(item as ItemFn);

    // Obtain the description
    let description =
        find_param_str("description", &args).unwrap_or_else(|| format!("{}()", item.sig.ident));

    // Wrap function with stopwatch
    item.block.stmts.insert(
        0,
        parse_quote! {
            let ___took = took::Timer::new();
        },
    );
    item.block.stmts.push(parse_quote! {
        eprintln!("{} took {}", #description, ___took);
    });

    // Convert modified item back into token stream
    quote!(#item).into()
}

/// Find a string value for an attribute parameter.
///
/// This function wals through a list of `NestedMeta` parsed from an attributes token stream.
/// It then attempts to find a key/value pair having `name`, and returns a string value.
///
/// If the parameter was not found, or if the value wasn't a string, `None` is returned.
fn find_param_str(name: &str, args: &[NestedMeta]) -> Option<String> {
    args.iter()
        .filter_map(|arg| match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => Some(nv),
            _ => None,
        })
        .filter_map(|arg| {
            // Match the parameter identifier
            match arg.path.segments.first() {
                Some(segment) if segment.ident == name => {}
                _ => return None,
            }

            // Get the parameter string value
            match &arg.lit {
                Lit::Str(value) => Some(value.value()),
                _ => panic!("parameter {0} must have string value: {0} = \"abc\"", name),
            }
        })
        .next()
}
