use std::{collections::HashMap, convert::Infallible};

use domorust_macros::route;
use warp::reply::{self, Reply, Response};
use warp::http::StatusCode;

use crate::server::responses::RequestError;

#[route(path=("domorust-api" / "themes"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_themes(_params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	Ok(warp::reply::json(&crate::server::responses::get_themes()))
}
#[route(path=("domorust-api" / "themes" / "current"), method="GET", query_params=false, needed_rights=0)]
pub async fn get_theme() -> Result<Response, Infallible> {
	//let current_theme = db::settings::get_setting_string("WebTheme");
	Ok(reply::with_status(reply::json(&RequestError::new("GetTheme","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
#[route(path=("domorust-api" / "themes" / "current"), method="PUT", query_params=true, needed_rights=2)]
pub async fn set_theme(_params: HashMap<String, String>) -> Result<Response, Infallible> {
	Ok(reply::with_status(reply::json(&RequestError::new("SetTheme","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
