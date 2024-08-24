use std::{collections::HashMap, convert::Infallible};

use domorust_macros::route;
use warp::reply::{self, Reply};
use warp::http::StatusCode;

use crate::db;
use crate::server::responses::{RequestError, RequestResult};

#[route(path=("domorust-api" / "events_scripts"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_events_scripts(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::events_scripts::get_events_scripts(params) {
		Ok(res) => {
			Ok(reply::json(&RequestResult::new("GetEventScript", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetEventScript",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "events_scripts"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_events_script(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::events_scripts::add_events_script(params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("AddEventScript", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddEventScript", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "events_scripts" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_events_script(idx: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::events_scripts::update_events_script(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("UpdateEventScript", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateEventScript", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "events_scripts" / usize), method="DELETE", query_params=true, needed_rights=2)]
pub async fn delete_events_script(idx: usize, _params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::events_scripts::delete_events_script(idx) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("DeleteEventScript", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteEventScript", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "events_scripts" / "devices_current_status"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_devices_current_status(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::events_scripts::get_devices_current_status(params) {
		Ok(res) => {
			let mut req=RequestResult::new("GetCurrentStatus", res);
			req.additional_fields.insert("interpreters".to_string(), "Blockly:Lua:dzVents:Python".to_string());
			Ok(warp::reply::json(&req).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetCurrentStatus", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
