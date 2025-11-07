use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn profile_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_sig = &fn_item.sig;
    let fn_block_stmts = &fn_item.block.stmts;

    let expanded = quote! {
        #fn_sig {
            use tuff_core;
            let __idx = {
                const __CALL_SITE: tuff_core::CallSite = tuff_core::CallSite::new(file!(), line!(), column!());
                tuff_core::Profiler::get_or_insert(__CALL_SITE)
            };
            let __block = tuff_core::ProfileBlock::new("", __idx);

            #(#fn_block_stmts)*
        }
    };

    expanded.into()
}
