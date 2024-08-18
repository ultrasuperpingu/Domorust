extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod common;
mod hashmap_instances;
mod sql_query;
mod sql_row_fields;
mod sql_row_instances;


#[proc_macro_derive(FromSqlRowFields, attributes(name_column_idx, value_column_idx, name_column_name, value_column_name, skip_field, name_value, table_name))]
pub fn derive_fromsql_row_fields(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	sql_row_fields::expand_derive_fromsql_row_fields(&input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

#[proc_macro_derive(ToSqlRowFields, attributes(name_column_name, value_column_name, skip_field, table_name, param_name))]
pub fn derive_tosql_row_fields(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	sql_row_fields::expand_derive_tosql_row_fields(&input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

#[proc_macro_derive(FromSqlRow, attributes(column_name, skip_field, table_name))]
pub fn derive_fromsql_row(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	sql_row_instances::expand_derive_fromsql_row(&input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

#[proc_macro_derive(FromSqlTable, attributes(column_name, skip_field, table_name))]
pub fn derive_fromsql_table(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	sql_row_instances::expand_derive_fromsql_table(&input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}


#[proc_macro_derive(ToSqlQuery, attributes(column_name, skip_field, primary_key, table_name))]
pub fn derive_sql_query(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	sql_query::expand_derive_sql_query(&input)
	.unwrap_or_else(syn::Error::into_compile_error)
	.into()
}

#[proc_macro_derive(FromHashMap, attributes(param_name, skip_field))]
pub fn derive_from_hashmap(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	hashmap_instances::expand_derive_from_hashmap(&input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

