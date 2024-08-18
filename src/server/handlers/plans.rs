use std::collections::HashMap;
use std::convert::Infallible;

use warp::reply::{self, Reply};

use crate::db;
use crate::server::responses::{RequestError, RequestResult, RequestResult2};

pub async fn get_floorplans(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::get_floorplans(params) {
		Ok(devices) => {
			match db::settings::get_floorplans_settings() {
				Ok(config) => {
					Ok(reply::json(&RequestResult2::new("GetFloorPlan", devices, config)).into_response())
				},
				Err(e) => {
					Ok(reply::with_status(reply::json(&RequestError::new("GetFloorPlan", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
				}
			}
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetFloorPlan", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_floorplan(idx:usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::get_floorplan(idx, params) {
		Ok(fplan) => {
			Ok(reply::json(&RequestResult::new("Plans", vec![fplan])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_floorplan_image(idx:usize) -> Result<impl warp::Reply, Infallible> {
	match db::plans::get_floorplan_image(idx) {
		Ok(data) => {
			Ok(reply::with_header(data, "Content-Type", "image/png").into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn get_plans(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::get_plans(params) {
		Ok(devices) => {
			Ok(reply::json(&RequestResult::new("Plans", devices)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_plan(idx:usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::get_plan(idx, params) {
		Ok(plan) => {
			Ok(reply::json(&RequestResult::new("Plans", vec![plan])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn add_plan(flooridx:usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::add_plan(flooridx, params) {
		Ok(()) => {
			Ok(reply::json(&RequestResult::<String>::new("Plans", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn update_plan(idx:usize,params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::update_plan(idx, params) {
		Ok(()) => {
			Ok(reply::json(&RequestResult::<String>::new("UpdatePlan", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdatePlan", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn delete_plan(idx:usize) -> Result<impl warp::Reply, Infallible> {
	match db::plans::delete_plan(idx) {
		Ok(()) => {
			Ok(reply::json(&RequestResult::<String>::new("UpdatePlan", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdatePlan", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_plan_devices(idx:usize,params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::plans::get_plan_devices(idx, params) {
		Ok(res) => {
			Ok(reply::json(&RequestResult::new("GetPlanDevices", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetPlanDevices", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
