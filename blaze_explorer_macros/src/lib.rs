extern crate proc_macro;

use blaze_explorer_lib::app::App;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Try dropping the poupup as a first action in the function body.
#[proc_macro_attribute]
pub fn quit_popup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // inject `app.try_drop_popup();` at the start of the function body
    let stmts = &mut input_fn.block.stmts;
    let injected: syn::Stmt = syn::parse_quote! {
        self.try_drop_popup();
    };
    stmts.insert(0, injected);

    TokenStream::from(quote! {
        #input_fn
    })
}
