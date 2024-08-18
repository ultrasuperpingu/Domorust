

use proc_macro2::TokenStream;
use quote::quote;

use crate::common::read_struct;
pub fn expand_derive_from_hashmap(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
	let stru=read_struct(input)?;
	let struct_name = stru.ident.unwrap();
	let fields_iter = stru.fields.iter().map(|f| {
		let field_ident=f.ident.unwrap();
		let param_name=&f.param_name;
		if !param_name.is_empty() && !f.skip {
			if f.option {
				quote!{
					if params.contains_key(#param_name) {
						let param_val = params.get(#param_name).ok_or::<Box<dyn std::error::Error>>("Can't parse hashmap value".into())?.parse();
						if let Ok(param_val) = param_val {
							res.#field_ident=Some(param_val);
						}
					}
				}
			} else {
				quote!{
					if params.contains_key(#param_name) {
						res.#field_ident=params.get(#param_name).ok_or::<Box<dyn std::error::Error>>("Can't parse hashmap value".into())?.parse()?;
					}
				}
			}
		} else {
			quote!{}
		}
	}).filter(|to| {!to.is_empty()});//.collect();
	let expanded = quote! {
		impl FromHashMap for #struct_name {
			fn from_hashmap(params: &std::collections::HashMap<String,String>) -> Result<Self, Box<dyn std::error::Error>> {
				let mut res = Self::default();
				#(#fields_iter)*
				Ok(res)
			}
		}
	};

	Ok(expanded)
}