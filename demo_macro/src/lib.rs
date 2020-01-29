extern crate proc_macro;

#[proc_macro]
pub fn print_things(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("Break rust-analyzer!");

    tokens
}