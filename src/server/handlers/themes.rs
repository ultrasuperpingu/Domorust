use std::{collections::HashMap, convert::Infallible};

use domorust_models::domorust::Domorust;
use warp::reply::{self, Reply, Response};
use warp::http::StatusCode;

use crate::server::responses::RequestError;


pub async fn get_themes(domorust: Domorust, _params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	Ok(warp::reply::json(&crate::server::responses::get_themes(domorust.cli)))
}

pub async fn get_theme() -> Result<Response, Infallible> {
	//let current_theme = db::settings::get_setting_string("WebTheme");
	Ok(reply::with_status(reply::json(&RequestError::new("GetTheme","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}

pub async fn set_theme(_params: HashMap<String, String>) -> Result<Response, Infallible> {
	Ok(reply::with_status(reply::json(&RequestError::new("SetTheme","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
