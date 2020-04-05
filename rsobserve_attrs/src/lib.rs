#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate darling;

use crate::darling::FromMeta;
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
pub fn observe(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(metadata as syn::AttributeArgs);

    let args: MacroArgs = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(err) => {
            return proc_macro::TokenStream::from(err.write_errors());
        }
    };

    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let vis = input_fn.vis;
    let sig = input_fn.sig;
    let block = input_fn.block;
    quote!(
        #vis #sig {
            #block
        }
    )
    .into()
}
