
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::spanned::Spanned;

use crate::common::read_struct;

pub fn expand_derive_fromsql_row_fields(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
	let stru=read_struct(input)?;
	if stru.table_name.is_empty() {
		return Err(syn::Error::new(stru.ident.span(), "Table name not specified"));
	}
	let struct_name = stru.ident.ok_or(syn::Error::new(stru.ident.span(), "Struct does not have ident"))?;
	let mut query_map_body=TokenStream::new();
	let mut field_colname_body=TokenStream::new();
	let mut first = true;
	let name_col_index=stru.name_col_index;
	let name_col_name=stru.name_col_name;
	let default_col_index=stru.default_col_index;
	let default_col_name=&stru.default_col_name;
	
	let table = stru.table_name;
	for f in &stru.fields {
		if !first {
			query_map_body.append_all(quote!{else});
			field_colname_body.append_all(quote!{else});
		}
		first = false;
		let field_ident=f.ident.unwrap();
		let field_name = &f.name_value;
		let col_index = f.column_index;
		let value_column_name = &f.value_column_name;
		let col_name = if !value_column_name.is_empty() {
			value_column_name
		} else if !default_col_name.is_empty() {
			default_col_name
		} else {
			field_name
		};
		field_colname_body.append_all(quote!{
			if field_name == #field_name {
				return Ok(#col_name.to_string())
			}
		});
		if name_col_index != usize::MAX {
			if col_index != usize::MAX {
				query_map_body.append_all(quote! {
					if row.get::<usize,String>(#name_col_index)? == #field_name {
						res.#field_ident=row.get(#col_index)?;
					}
				})
			} else if !value_column_name.is_empty() {
				query_map_body.append_all(quote! {
					if row.get::<usize,String>(#name_col_index)? == #field_name {
						res.#field_ident=row.get(#value_column_name)?;
					}
				});
			} else if default_col_index != usize::MAX {
				query_map_body.append_all(quote! {
					if row.get::<usize,String>(#name_col_index)? == #field_name {
						res.#field_ident=row.get(#default_col_index)?;
					}
				});
			} else if !default_col_name.is_empty() {
				query_map_body.append_all(quote! {
					if row.get::<usize,String>(#name_col_index)? == #field_name {
						res.#field_ident=row.get(#default_col_name)?;
					}
				});
			} else {
				return Err(syn::Error::new(stru.ident.span(), "Neither `default_col_name` or `default_col_index` on struct or value_column_name or value_column_name on field is set"))
			}
		} else if !name_col_name.is_empty() {
			if f.column_index != usize::MAX {
				query_map_body.append_all(quote! {
					if row.get::<&str,String>(#name_col_name)? == #field_name {
						res.#field_ident=row.get(#col_index)?;
					}
				});
			} else if !value_column_name.is_empty() {
				query_map_body.append_all(quote! {
					if row.get::<&str,String>(#name_col_name)? == #field_name {
						res.#field_ident=row.get(#value_column_name)?;
					}
				});
			} else if default_col_index != usize::MAX {
				query_map_body.append_all(quote! {
					if row.get::<&str,String>(#name_col_name)? == #field_name {
						res.#field_ident=row.get(#default_col_index)?;
					}
				});
			} else if !default_col_name.is_empty() {
				query_map_body.append_all(quote! {
					if row.get::<&str,String>(#name_col_name)? == #field_name {
						res.#field_ident=row.get(#default_col_name)?;
					}
				});
			} else {
				return Err(syn::Error::new(stru.ident.span(), "Neither `default_column_name` or `default_column_index` on struct or value_column_name or value_column_name on field is set"))
			}
			//println!("{:?}",query_map_body);
		} else {
			return Err(syn::Error::new(stru.ident.span(), "Neither `name_column_name` or `name_column_index` is set on the structure"))
		}
	}
	let expanded = quote! {
		impl FromSqlRowFields for #struct_name {
			fn read(connection:&rusqlite::Connection) -> Result<Self, Box<dyn std::error::Error>> {
				let query = format!("SELECT * FROM {}", #table);
				let mut res=Self::default();
				let mut stmt = connection.prepare(query.as_str())?;
				let mut location="".to_string();
				let rows_iter = stmt.query_map([], |row| {
					#query_map_body
					Ok(())
				});
				for _ in rows_iter? {
				}
				Ok(res)
			}
			fn get_column_name(field_name:&String) -> Result<String, Box<dyn std::error::Error>> {
				#field_colname_body
				Err("Field not found ".into())
			}
		}
	};
	//println!("{:?}",expanded.to_string());
	Ok(expanded)
}

pub fn expand_derive_tosql_row_fields(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
	let stru=read_struct(input)?;
	if stru.table_name.is_empty() {
		return Err(syn::Error::new(stru.ident.span(), "Table name not specified"));
	}
	let struct_name = stru.ident.ok_or(syn::Error::new(stru.ident.span(), "Struct does not have ident"))?;
	//let mut select_test=TokenStream::new();
	//let mut params_test=TokenStream::new();
	let mut build_update_query_str=TokenStream::new();
	let mut build_update_query_instance_str=TokenStream::new();
	//let mut first = true;
	//let name_col_index=stru.name_col_index;
	let name_col_name=stru.name_col_name;
	//let default_col_index=stru.default_col_index;
	let default_col_name=&stru.default_col_name;
	let table = stru.table_name;

	/*#[cfg(debug_assertions)]
	params_test.append_all(quote!{
		for (key, _value) in &params {
			let select_query=concat!("SELECT * FROM ",#table, " WHERE ",#name_col_name,"=?1;");
			let res = connection.query_row(select_query, [key], |row| {Ok(())});
			if let Err(e) = res {
				eprintln!(concat!("Error finding {} value in table ",#table,": {}"), key, e);
			}
		}
	});*/

	for f in &stru.fields {
		//if !first {
			//build_query_str.append_all(quote!{else});
		//}
		//first = false;
		let field_ident=f.ident.unwrap();
		let field_name = &f.name;
		//let col_index = f.col_index;
		let value_column_name = &f.value_column_name;
		
		if name_col_name.is_empty() {
			return Err(syn::Error::new(f.ident.span(), "name_col_name is mandatory for ToSqlRowField"))
		} else {
			let val_col_name= if !value_column_name.is_empty() {
				value_column_name
			} else if !default_col_name.is_empty() {
				default_col_name
			} else {
				return Err(syn::Error::new(f.ident.span(), "default_col_name on struct or colum_name on field is mandatory for ToSqlRowField"))
			};
			if f.need_quote {
				build_update_query_str.append_all(quote! {
					if params.contains_key(#field_name) {
						query+=format!(concat!("UPDATE ",#table, " SET ", #val_col_name,"={} WHERE ",#name_col_name,"=\"", #field_name, "\";"), "\"".to_string() + params.get(#field_name).unwrap().as_str() + "\"").as_str();
					}
				});
				build_update_query_instance_str.append_all(quote! {
					query+=format!(concat!("UPDATE ",#table, " SET ", #val_col_name,"={} WHERE ",#name_col_name,"=\"", #field_name, "\";"), "\"".to_string() + self.#field_ident.to_string().as_str() + "\"").as_str();
				});
			} else {
				build_update_query_str.append_all(quote! {
					if params.contains_key(#field_name) {
						query+=format!(concat!("UPDATE ",#table, " SET ", #val_col_name,"={} WHERE ",#name_col_name,"=\"", #field_name, "\";"), params.get(#field_name).unwrap()).as_str();
					}
				});
				build_update_query_instance_str.append_all(quote! {
					query+=format!(concat!("UPDATE ",#table, " SET ", #val_col_name,"={} WHERE ",#name_col_name,"=\"", #field_name, "\";"), self.#field_ident).as_str();
				});
			}
			/*#[cfg(debug_assertions)]
			select_test.append_all(quote!{
				if params.contains_key(#field_name) {
					let select_query=concat!("SELECT * FROM ",#table, " WHERE ",#name_col_name,"=\"", #field_name, "\";");
					let res = connection.query_row(select_query, [], |row| {Ok(())});
					if let Err(e) = res {
						eprintln!(concat!("Error finding ",#field_name," value in table ",#table,": {}"), e);
					}
				}
			});*/
		}
	}
	let expanded = quote! {
		impl ToSqlRowFields for #struct_name {
			fn write(connection:&rusqlite::Connection, params:HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>> {
				//#select_test
				//#params_test
				let mut query = "".to_string();
				#build_update_query_str
				let written = connection.execute_batch(query.as_str())?;
				//TODO: check nb written
				Ok(())
			}
			fn write_instance(&self, connection:&rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
				let mut query = "".to_string();
				#build_update_query_instance_str
				let written = connection.execute_batch(query.as_str())?;
				//TODO: check nb written
				Ok(())
			}
		}
	};
	//println!("{:?}",expanded.to_string());
	Ok(expanded)
}