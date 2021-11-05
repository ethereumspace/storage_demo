extern crate proc_macro;
use proc_macro::TokenStream;
use event;

#[proc_macro_attribute]
pub fn update(attr: TokenStream, item: TokenStream) -> TokenStream {
    pr
    event::createEvent(item.to_string().clone());
    item
}

