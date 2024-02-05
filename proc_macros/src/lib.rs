use class::class_impl;
use proc_macro::TokenStream;

mod class;
mod classinfo;

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    class_impl(input)
}