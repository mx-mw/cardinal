use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;
    let generics = input.generics;
    let (impl_generics, 
		ty_generics, 
		where_clause) = generics.split_for_impl();

	
    let output = quote! {
		impl #impl_generics Component for #name #ty_generics #where_clause {
			fn init(&mut self) {
		
			}
			fn update(&mut self) {
				
			}
			fn timer(&mut self) {
				
			}
			fn render(&mut self) {
				
			}
		}
	};

    TokenStream::from(output)
}
