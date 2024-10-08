use std::{collections::HashMap, convert::Infallible};
use domorust_macros::route;
use domorust_models::hardware::HardwareData;
use warp::reply::{self, Reply};
use warp::http::StatusCode;


use crate::server::responses::{RequestError, RequestResult};
use crate::db::hardwares::{get_hardware_data, get_hardwares_data};

#[route(path=("domorust-api" / "hardwares"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_hardwares(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	let hws = get_hardwares_data(params);
	match hws {
		Ok(hws) => {
			let res = RequestResult::<HardwareData>::new( "GetHardwares", hws);
			Ok(warp::reply::with_status(warp::reply::json(&res),warp::http::StatusCode::OK))
		},
		Err(e) => {
			let res = RequestError::new("GetHardwares", Box::new(e));
			Ok(warp::reply::with_status(warp::reply::json(&res),warp::http::StatusCode::INSUFFICIENT_STORAGE))
		},
	}
}
#[route(path=("domorust-api" / "hardwares" / usize), method="GET", query_params=false, needed_rights=0)]
pub async fn get_hardware(idx : usize) -> Result<impl warp::Reply, Infallible> {
	let hd=get_hardware_data(idx);
	match hd {
		Ok(hd) => {
			let res = RequestResult::<HardwareData>::new("gethardware", vec![hd]);
			Ok(warp::reply::with_status(warp::reply::json(&res),warp::http::StatusCode::OK))
		},
		Err(e) => {
			let res = RequestError::new("GetHardware", Box::new(e));
			Ok(warp::reply::with_status(warp::reply::json(&res),warp::http::StatusCode::INSUFFICIENT_STORAGE))
		}
	}
}
#[route(path=("domorust-api" / "hardwares"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_hardware(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	println!("add_hardware {:?}", params);
	Ok(reply::with_status(reply::json(&RequestError::new("AddHardware","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR))
}
#[route(path=("domorust-api" / "hardwares" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_hardware(idx: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	println!("update_hardware {} {:?}", idx, params);
	Ok(reply::with_status(reply::json(&RequestError::new("UpdateHardware","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR))
}
#[route(path=("domorust-api" / "hardwares" / usize), method="DELETE", query_params=true, needed_rights=2)]
pub async fn delete_hardware(idx: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	println!("delete_hardware {} {:?}", idx, params);
	Ok(reply::with_status(reply::json(&RequestError::new("DeleteHardware","Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR))
}
