#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate darling;


use proc_macro;

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    with_result: bool,
    #[darling(default)]
    without_result: bool,
    #[darling(default)]
    namespace: Option<String>,
    #[darling(default)]
    id: Option<String>,
}

#[proc_macro_attribute]
pub fn observe(metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(metadata as syn::AttributeArgs);
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    quote!().into()
    //proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
