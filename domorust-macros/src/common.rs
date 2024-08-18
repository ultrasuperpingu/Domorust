use proc_macro2::Ident;
use syn::{spanned::Spanned, Type, Data, Fields, LitInt, LitStr};

#[derive(Debug)]
pub(crate) struct Structure<'a> {
	pub name:String,
	pub ident:Option<&'a Ident>,
	pub table_name:String,
	pub name_col_index:usize,
	pub name_col_name:String,
	pub default_col_name:String,
	pub default_col_index:usize,
	pub primary_index:usize,
	pub fields: Vec<MyField<'a>>
}
impl<'a> Default for Structure<'a> {
	fn default() -> Self {
		Self {
 			name: Default::default(),
			ident: Default::default(),
			table_name: Default::default(),
			name_col_index: usize::MAX,
			name_col_name: Default::default(),
			default_col_name: Default::default(),
			default_col_index: usize::MAX,
			primary_index: usize::MAX,
			fields: Default::default()
		}
	}
}
#[derive(Debug)]
pub(crate) struct MyField<'a> {
	pub name:String,
	pub ident:Option<&'a Ident>,
	pub ty:Option<&'a Type>,
	pub value_column_name:String,
	pub column_name:String,
	pub column_index:usize,
	pub name_value : String,
	pub param_name:String,
	pub primary_key:bool,
	pub skip:bool,
	pub need_quote:bool,
	pub option:bool,
}

impl<'a> Default for MyField<'a> {
	fn default() -> Self {
		Self {
			name: Default::default(),
			ident: Default::default(),
			ty: Default::default(),
			value_column_name: Default::default(),
			column_name: Default::default(),
			column_index: usize::MAX,
			name_value: Default::default(),
			param_name: Default::default(),
			primary_key: false,
			skip: false,
			need_quote: false,
			option: false,
		}
	}
}

pub(crate) fn read_struct(input: &syn::DeriveInput) -> syn::Result<Structure> {
	//TODO: return an error instead of panic
	let mut stru=Structure::default();
	stru.ident = Some(&input.ident);
	stru.name = input.ident.to_string();

	for a in &input.attrs {
		if a.meta.path().is_ident("skip_field") {
			return Err(syn::Error::new(a.span(),"`skip_field` not allowed on a struct, only fields"));
		}
		if a.meta.path().is_ident("column_name") {
			return Err(syn::Error::new(a.span(),"`column_name` not allowed on a struct, only fields"));
		}
		if a.meta.path().is_ident("name_value") {
			return Err(syn::Error::new(a.span(),"`name_value` not allowed on a struct, only fields"));
		}
		if a.meta.path().is_ident("params_name") {
			return Err(syn::Error::new(a.span(),"`params_name` not allowed on a struct, only fields"));
		}
		if a.meta.path().is_ident("value_column_idx") {
			let expr: LitInt = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `value_column_idx` attribute expects integer literal: {}", e))})?;
			stru.default_col_index = expr.base10_parse().map_err(|e| {syn::Error::new(a.span(), format!("The `value_column_idx` attribute expects integer literal: {}", e))})?;
		}
		if a.meta.path().is_ident("value_column_name") {
			let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `value_column_name` attribute expects string literal {}", e))})?;
			stru.default_col_name = expr.value();
		}
		if a.meta.path().is_ident("name_column_idx") {
			let expr: LitInt = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `name_column_idx` attribute expects integer literal {}", e))})?;
			stru.name_col_index = expr.base10_parse().map_err(|e| {syn::Error::new(a.span(), format!("The `name_column_idx` attribute expects integer literal {}", e))})?;
		}
		if a.meta.path().is_ident("name_column_name") {
			let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(),format!("The `name_column_name` attribute expects integer literal: {}", e))})?;
			stru.name_col_name = expr.value();
		}
		if a.meta.path().is_ident("table_name") {
			let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `table_name` attribute expects integer literal: {}", e))})?;
			stru.table_name = expr.value();
			if !stru.table_name
				.chars()
				.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
					return Err(syn::Error::new(a.span(),"`table_name` must be [a-zA-Z0-9_-]+"))
			}
		}
	}

	let fields = match &input.data {
		Data::Struct(data_struct) => match &data_struct.fields {
			Fields::Named(fields_named) => {
				&fields_named.named
			},
			_ => return Err(syn::Error::new(input.span(),"Only structs with named fields are supported")),
		},
		_ => return Err(syn::Error::new(input.span(),"Only structs are supported")),
	};
	let fields_iter = fields.iter().map(|field| {
		let mut myfield = MyField::default();
		myfield.ident = field.ident.as_ref();
		myfield.name = (&myfield.ident).unwrap().to_string();
		myfield.ty = Some(&field.ty);
		myfield.need_quote = is_need_quote(&field.ty);
		myfield.option = is_option(&field.ty);
		//println!("field {}",myfield.name);
		for a in &field.attrs {
			//println!("\tattr {}",a.meta.path().get_ident().unwrap().to_string());
			//TODO: how export error in closure without panic?
			if a.meta.path().is_ident("skip_field") {
				myfield.skip=true;
				continue;
			}
			if a.meta.path().is_ident("value_column_name") {
				let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `value_column_name` attribute expects string literal {}", e))}).unwrap();
				myfield.value_column_name = expr.value();
			}
			if a.meta.path().is_ident("column_name") {
				let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `column_name` attribute expects string literal: {}", e))}).unwrap();
				myfield.column_name = expr.value();
			}
			if a.meta.path().is_ident("name_value") {
				let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `name_value` attribute expects string literal: {}", e))}).unwrap();
				myfield.name_value = expr.value();
			}
			if a.meta.path().is_ident("param_name") {
				let expr: LitStr = a.parse_args().map_err(|e| {syn::Error::new(a.span(), format!("The `param_name` attribute expects string literal: {}", e))}).unwrap();
				myfield.param_name = expr.value();
			}
			if a.meta.path().is_ident("primary_key") {
				myfield.primary_key = true;
			}
			if a.meta.path().is_ident("value_column_idx") {
				let expr: LitInt = a.parse_args().map_err(|e| {syn::Error::new(a.span(),format!("The `name_column_name` attribute expects integer literal: {}", e))}).unwrap();
				myfield.column_index = expr.base10_parse().map_err(|e| {syn::Error::new(a.span(),format!("The `name_column_name` attribute expects integer literal: {}", e))}).unwrap();
			}
		}
		myfield
	});
	for mut f in fields_iter {
		if f.column_name.is_empty() {
			f.column_name = f.name.clone();
		}
		if f.name_value.is_empty() {
			f.name_value = f.name.clone();
		}
		if f.param_name.is_empty() {
			f.param_name = f.name.clone();
		}
		stru.fields.push(f);
	}
	for f in stru.fields.iter().enumerate() {
		if f.1.primary_key {
			if stru.primary_index == usize::MAX {
				stru.primary_index=f.0;
			} else {
				return Err(syn::Error::new(f.0.span(), "Struct can only have one primary key"));
			}
		}
	}
	//println!("{:?}", stru);
	Ok(stru)
}

//TODO: Be more robust
fn is_need_quote(ty: &syn::Type) -> bool {
	let path= match *ty {
		syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
		_ => None,
	};
	if path.is_none() {
		return true;
	}
	let path = path.unwrap();
	let idents_of_path = path
		.segments
		.iter()
		.into_iter()
		.fold(String::new(), |mut acc, v| {
			acc.push_str(&v.ident.to_string());
			acc
		});
	//println!("is_need_quote extracted type name: {}",idents_of_path);
	idents_of_path.ends_with("String") || idents_of_path.ends_with("Time")
	|| idents_of_path.ends_with("Date") || idents_of_path.ends_with("GPSCoord")
}

//TODO: Be more robust
fn is_option(ty: &syn::Type) -> bool {
	let path= match *ty {
		syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
		_ => None,
	};
	if path.is_none() {
		return false;
	}
	let path = path.unwrap();
	let idents_of_path = path
		.segments
		.iter()
		.into_iter()
		.fold(String::new(), |mut acc, v| {
			acc.push_str(&v.ident.to_string());
			acc
		});
	//println!("is_need_quote extracted type name: {}",idents_of_path);
	idents_of_path.starts_with("Option")
}