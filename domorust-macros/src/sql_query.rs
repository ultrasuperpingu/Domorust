
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;

use crate::common::read_struct;

pub fn expand_derive_sql_query(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
	let stru = read_struct(&input)?;
	let struct_name = stru.ident;
	let table = stru.table_name;
	let primary = stru.fields[stru.primary_index].ident;
	let update_fields_assigns = stru.fields.iter().map(|f| {
		let col_name = &f.column_name;
		quote!{
			concat!(" ",stringify!(#col_name),"=:",stringify!(#col_name), ",")
		}
	}).filter(|to| {!to.is_empty()});//.collect();
	let update_fields_assigns = quote!{#(#update_fields_assigns)+*};
	let primary_index = proc_macro2::TokenStream::from_str(&format!("{}", stru.primary_index)).unwrap();
	
	let update_fields_tuple = stru.fields.iter().map(|field| {
		let field_ident = &field.ident;
		//let field_name = field_ident.as_ref().unwrap().to_string();
		
		quote!{
			&self.#field_ident
		}
	}).filter(|to| {!to.is_empty()});

	let update_fields_name_list_add = stru.fields.iter().map(|f| {
		let col_name = &f.column_name;
		quote!{
			concat!(" ",stringify!(#col_name), ",")
		}
	}).filter(|to| {!to.is_empty()});//.collect();
	let update_fields_name_list_add = quote!{#(#update_fields_name_list_add)+*};
	let update_fields_index_list_add = stru.fields.iter().map(|f| {
		let col_name = &f.column_name;
		quote!{
			concat!(" :",stringify!(#col_name), ",")
		}
	}).filter(|to| {!to.is_empty()});//.collect();
	let update_fields_index_list_add = quote!{#(#update_fields_index_list_add)+*};
	let update_fields_tuple_add = stru.fields.iter().map(|f| {
		let field_ident = &f.ident;
		quote!{
			&self.#field_ident
		}
	}).filter(|to| {!to.is_empty()});
	// TODO: concat iter tokestream
	let expanded = quote! {
		impl ToSqlQuery for #struct_name {
			fn update_query(&self, connection:&rusqlite::Connection) -> Result<(), rusqlite::Error> {
				//"UPDATE ".to_owned()+table+" SET "+"#(#fields_iter)*" + "WHERE ID="
				//let query = "UPDATE ".to_string()+ table + " SET" + #(#update_fields_assigns)+*.trim_end_matches(',');
				let query = "UPDATE ".to_string() + #table + " SET" + #update_fields_assigns.trim_end_matches(',');
				let query = query + " WHERE " + stringify!(#primary) + "=$" + stringify!(#primary_index);
				let params=(#(#update_fields_tuple),*);
				let res = connection.execute(query.as_str(), params)?;
				Ok(())
			}
			fn add_query(&self, connection:&rusqlite::Connection) -> Result<(), rusqlite::Error> {
				let query = "INSERT INTO ".to_string() + #table + " ("+#update_fields_name_list_add.trim_end_matches(',')+") VALUES ("+#update_fields_index_list_add.trim_end_matches(',')+")";
				let res = connection.execute(query.as_str(), ( #(#update_fields_tuple_add),*) )?;
				Ok(())
			}
		}
	};
	
	Ok(expanded)
}