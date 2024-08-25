#![forbid(unsafe_code)]
extern crate proc_macro;

use std::str::FromStr;

use common::RouteParams;
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Ident};

use quote::{quote, TokenStreamExt};

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

#[proc_macro_derive(FromSqlTable, attributes(column_name, skip_field, table_name, primary_key))]
pub fn derive_fromsql_table(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	sql_row_instances::expand_derive_fromsql_table(&input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}


#[proc_macro_derive(ToSqlQuery, attributes(column_name, skip_field, primary_key, table_name, skip_field_sql_write))]
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

#[proc_macro_attribute]
pub fn add_session(_attrs: TokenStream, item: TokenStream) -> TokenStream {
	let input_fn = parse_macro_input!(item as syn::ItemFn);

	let fn_sig = &input_fn.sig.clone();
	let fn_vis = &input_fn.vis.clone();
	let fn_block = &mut input_fn.block.clone();
	//TODO: do test_auth before request
	let expanded = quote! {
		#fn_vis #fn_sig {
			#fn_block
			.and(warp_sessions::request::with_session(
				crate::server::session_store::MySessionStore::new(),
				(*crate::server::session_store::COOKIE).clone(),
			).and(warp::addr::remote().and(crate::server::with_const(2))))
			.and_then(crate::server::handlers::login::test_auth)
			.untuple_one()
			.and_then(warp_sessions::reply::with_session)
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn route(attrs: TokenStream, item: TokenStream) -> TokenStream {
	let input_fn = &parse_macro_input!(item as syn::ItemFn);
	let mut args = RouteParams::default();
	let route_params_parser = syn::meta::parser(|meta| args.parse(meta));
	parse_macro_input!(attrs with route_params_parser);
	let func_name = &input_fn.sig.ident;
	let func_params = &input_fn.sig.inputs;
	let mut func_params_call: Punctuated<syn::Ident, syn::token::Comma> = Punctuated::new();
	for  i in &input_fn.sig.inputs {
		if let syn::FnArg::Typed(arg) = i {
			if let syn::Pat::Ident(arg_name) = arg.pat.as_ref() {
				func_params_call.push(arg_name.ident.clone());
			}
		}
	}
	let route_func_name = Ident::new(("route_".to_string() + input_fn.sig.ident.to_string().as_str()).as_str(), input_fn.sig.ident.span());
	let func_name_with_session = Ident::new((input_fn.sig.ident.to_string() + "_with_session").as_str(), input_fn.sig.ident.span());
	let path = args.path;
	let method_code = method_to_filter(&args.method);
	let query_params_code = query_params_filter(args.query_params);
	let query_form_code = query_form_filter(args.query_form);
	let comma = if func_params.is_empty() { quote!{} } else { quote!{,} };
	let rights = proc_macro2::TokenStream::from_str(&format!("{}",args.needed_rights)).unwrap();
	let test_auth = if let Some(test_auth)=args.custom_test_auth {
		quote!{#test_auth}
		//quote!{crate::server::handlers::login::test_auth}
	} else {
		quote!{crate::server::handlers::login::test_auth}
	};
	let mut res=quote! {
		pub(crate) fn #route_func_name() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
			use warp::Filter;
			warp::any()
			.and(
				warp::path!#path
				#method_code
				#query_params_code
				#query_form_code
			)
			.and(warp_sessions::request::with_session(
				crate::server::session_store::MySessionStore::new(),
				(*crate::server::session_store::COOKIE).clone(),
			)
			.and(
				warp::addr::remote()
				.and(crate::server::with_const(#rights))
				.and(warp::path::full())
			))
			.and_then(#func_name_with_session)
			.untuple_one()
			.and_then(warp_sessions::reply::with_session)
		}
		async fn #func_name_with_session(
			#func_params #comma
			store: warp_sessions::SessionWithStore<crate::server::session_store::MySessionStore>,
			socket: Option<std::net::SocketAddr>,
			needed_rights: i32,
			path: warp::path::FullPath
			) -> Result<(impl warp::reply::Reply, warp_sessions::SessionWithStore<crate::server::session_store::MySessionStore>), warp::Rejection>
		{
			println!("request path: {:?}", path);
			if let Err(reply) = #test_auth(&store, socket, needed_rights).await {
				println!("auth failed: {:?}", path);
				return Ok((reply.into_response(), store));
			}
		
			Ok((#func_name(#func_params_call).await?.into_response(), store))
		}
	};
	res.append_all(quote!{#input_fn});
	res.into()
}
fn method_to_filter(method: &str) -> proc_macro2::TokenStream {
	match method {
		"DELETE" => quote!{.and(warp::delete())},
		"GET" => quote!{.and(warp::get())},
		"PATCH" => quote!{.and(warp::patch())},
		"POST" => quote!{.and(warp::post())},
		"PUT" => quote!{.and(warp::put())},
		_ => quote!{}
	}
}
fn query_params_filter(params: bool) -> proc_macro2::TokenStream {
	match params {
		true => quote!{.and(warp::query::<HashMap<String, String>>())},
		_ => quote!{}
	}
}
fn query_form_filter(params: bool) -> proc_macro2::TokenStream {
	match params {
		true => quote!{.and(warp::multipart::form().max_length(500_000))},
		_ => quote!{}
	}
}

		