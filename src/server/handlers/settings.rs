use std::{collections::HashMap, convert::Infallible};
use domorust_macros::route;
use domorust_models::settings::Settings;
use domorust_models::FromSqlRowFields;
use warp::reply::{self, Reply};
use warp::http::StatusCode;

use crate::server::responses::{RequestError, RequestResult};
use crate::db;
use bytes::Buf;
use futures_util::TryStreamExt;

#[route(path=("domorust-api" / "settings"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_settings(params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::settings::get_settings(params) {
		Ok(res) => {
			Ok(reply::json(&res).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetSettings",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "settings" / String), method="GET", query_params=false, needed_rights=0)]
pub async fn get_setting(name: String) -> Result<impl Reply, Infallible> {
	let col_name=Settings::get_column_name(&name.to_string());
	if let Err(e) = col_name {
		return Ok(reply::with_status(reply::json(&RequestError::new("GetSetting",e)), StatusCode::NOT_FOUND).into_response())
	}
	let col_name = col_name.unwrap();
	if col_name == "nValue" {
		match db::settings::get_setting_int(name.as_str()) {
			Ok(res) => {
				Ok(warp::reply::json(&RequestResult::new("GetSetting", vec![res])).into_response())
			},
			Err(e) => {
				Ok(reply::with_status(reply::json(&RequestError::new("GetSetting",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
			}
		}
	} else if col_name == "sValue" {
		match db::settings::get_setting_string(name.as_str()) {
			Ok(res) => {
				Ok(warp::reply::json(&RequestResult::new("GetSetting", vec![res])).into_response())
			},
			Err(e) => {
				Ok(reply::with_status(reply::json(&RequestError::new("GetSetting",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
			}
		}
	} else {
		Ok(reply::with_status(reply::json(&RequestError::new("GetSetting","Invalid colum_name".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response())
	}
	
}
#[route(path=("domorust-api" / "settings"), method="PUT", query_params=true, query_form=true, needed_rights=2)]
pub async fn store_settings(_params: HashMap<String, String>, content: warp::multipart::FormData) -> Result<impl Reply, Infallible> {
	let test:HashMap<String,String>=content.and_then(|mut field| async move {
		let mut bytes: Vec<u8> = Vec::new();
		// field.data() only returns a piece of the content, you should call over it until it replies None
		while let Some(content) = field.data().await {
			let mut content = content.unwrap();
			while content.has_remaining() {
				bytes.push(content.get_u8());
			}
		}
		Ok((
			field.name().to_string(),
			//field.filename().unwrap_or("").to_string(),
			String::from_utf8_lossy(&bytes).to_string(),
		))
	}).try_collect().await.unwrap();
	match db::settings::store_settings(test) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("StoreSettings", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("StoreSettings",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "settings" / String), method="PUT", query_params=true, needed_rights=2)]
pub async fn set_setting(name: String, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	let val = params.get("value");
	if val.is_none() {
		return Ok(reply::with_status(reply::json(&RequestError::new("SetSetting","Not found".into())), StatusCode::BAD_REQUEST).into_response())
	}
	match db::settings::set_setting(&name, val.unwrap()) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("SetSetting", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("SetSetting",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
