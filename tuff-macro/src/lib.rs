use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn profile_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_sig = &fn_item.sig;
    let label = format!("fn::{}", fn_sig.ident);
    let fn_block_stmts = &fn_item.block.stmts;

    let expanded = quote! {
        #fn_sig {
            let __idx = {
                static __CALLSITE_ID: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                *__CALLSITE_ID.get_or_init(|| ::tuff::Profiler::next_id())
            };
            let __block = ::tuff::ProfileBlock::new(#label, __idx);

            #(#fn_block_stmts)*
        }
    };

    expanded.into()
}
