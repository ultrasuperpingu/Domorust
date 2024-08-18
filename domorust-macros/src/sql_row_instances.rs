
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use crate::common::read_struct;

pub fn expand_derive_fromsql_row(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
	let stru=read_struct(input)?;
	//println!("{:?}",stru.name);
	let struct_name = stru.ident.unwrap();
	let mut query_map_body=TokenStream::new();
	let mut first = true;
	for f in &stru.fields {
		if f.skip {
			continue;
		}
		if !first {
			query_map_body.append_all(quote!{,});
		}
		first = false;
		let field_ident=f.ident.unwrap();
		let col_name = &f.column_name;
		//println!("{}", col_name);
		query_map_body.append_all(
			quote!{
				#field_ident:row.get(#col_name)?
			}
		);
	}
	let expanded = quote! {
		impl FromSqlRow for #struct_name {
			fn build_from_row<'a>(row: &rusqlite::Row<'a>) -> Result<Self, rusqlite::Error> {
				let res=#struct_name {
					#query_map_body,
					..Default::default()
				};
				Ok(res)
			}
		}
	};
	//println!("{:?}",expanded.to_string());
	Ok(expanded)
}

pub fn expand_derive_fromsql_table(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
	let stru=read_struct(input)?;
	//println!("{:?}",stru.name);
	let struct_name = stru.ident.unwrap();
	//let mut query_map_body=TokenStream::new();
	if stru.table_name.is_empty() {
		return Err(syn::Error::new(stru.ident.unwrap().span(), "Table name not specified"));
	}
	let table = stru.table_name;
	let mut columns_list=String::new();
	let mut first=true;
	let mut where_clause = quote! {};
	where_clause.append_all(quote!{let mut first = true;});
	for f in &stru.fields {
		if f.skip {
			continue;
		}
		if !first {
			columns_list+=", ";
		}
		first = false;
		let param_name = &f.param_name;
		let col_name = &f.column_name;
		let need_quote = &f.need_quote;
		columns_list+="[";
		columns_list+=col_name;
		columns_list+="]";
		if !param_name.is_empty() {
			where_clause.append_all(quote! {
				if filters.contains_key(#param_name) {
					if first {
						first = false;
					} else {
						where_clause+=" AND ";
					}
					where_clause+=concat!(#col_name,"=");
					if #need_quote {
						where_clause+="\"";
						where_clause+=filters.get(#param_name).unwrap().as_str();
						where_clause+="\"";
					} else {
						where_clause+=filters.get(#param_name).unwrap().as_str();
					}
				}
			});
		}
	}
	if where_clause.is_empty() {
		where_clause.append_all(quote!{where_clause+="TRUE";});
	} else {
		where_clause.append_all(quote!{
			if where_clause.is_empty() {
				where_clause+="TRUE";
			}
		});
	}
	let expanded = quote! {
		impl FromSqlTable for #struct_name {
			fn build_from_table(connection:&rusqlite::Connection,
				filters:&std::collections::HashMap<String,String>)
					-> Result<Vec<Self>, rusqlite::Error> {
				let mut res=vec![];
				let mut where_clause = "".to_string();
				#where_clause;
				let query = format!(concat!("SELECT ",#columns_list," FROM ", #table, " WHERE {}"), where_clause);
				//println!("{}", query);
				let mut stmt = connection.prepare(query.as_str())?;
				let iter = stmt.query_map([], |row| {
					let item=#struct_name::build_from_row(row)?;
					Ok(item)
				})?;
				for item in iter {
					res.push(item?);
				}
				Ok(res)
			}
		}
	};

	Ok(expanded)
}
