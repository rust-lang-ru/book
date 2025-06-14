use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn здравствуй_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_здравствуй_macro(&ast)
}

// ANCHOR: here
fn impl_здравствуй_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn здравствуй_macro() {
                println!("Здравствуй, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
// ANCHOR_END: here
