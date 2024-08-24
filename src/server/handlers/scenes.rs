use std::{collections::HashMap, convert::Infallible};
use domorust_macros::route;
use domorust_models::scenes::SceneDevice;
use domorust_models::timers::Timer;
use warp::reply::{self, Reply};
use warp::http::StatusCode;

use crate::server::responses::{RequestError, RequestResult};
use crate::db;

#[route(path=("domorust-api" / "scenes"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_scenes(params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::get_scenes(params) {
		Ok(res) => {
			Ok(reply::json(&RequestResult::new("GetScenes", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetScenes",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_scene(params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	//type=command&param=addscene&name=df&scenetype=0
	match db::scenes::add_scene(params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("AddScene", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddScene",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_scene(idx: usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::update_scene(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("UpdateScene", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateScene",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize), method="DELETE", query_params=false, needed_rights=2)]
pub async fn delete_scene(idx: usize) -> Result<impl Reply, Infallible> {
	//type=command&param=deletescene&idx=2
	match db::scenes::delete_scene(idx) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("DeleteScene", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteScene", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "devices"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_scene_devices(idx :usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::get_scenes_devices(idx, params) {
		Ok(res) => {
			Ok(warp::reply::json(&RequestResult::<SceneDevice>::new("GetSceneDevices", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetSceneDevices",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "activations"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_scene_activations(idx:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::get_scenes_activations(idx, params) {
		Ok(res) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("GetSceneActivations", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetSceneActivations",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "timers"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_scene_timers(idx:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::get_scene_timers(idx, params) {
		Ok(res) => {
			Ok(warp::reply::json(&RequestResult::<Timer>::new("GetSceneTimers", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetSceneTimers",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "logs"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_scene_logs(idx:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::get_scene_logs(idx, params) {
		Ok(res) => {
			Ok(warp::reply::json(&RequestResult::new("GetSceneLogs", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetSceneLogs",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "devices"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_scene_device(idx:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::add_scene_device(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("GetSceneLogs", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddSceneDevice", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "activations"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_scene_activation(idx:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::add_scene_activation(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("GetSceneLogs", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddSceneActivation", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "scenes" / usize / "timers"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_scene_timer(idx:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::scenes::add_scene_timer(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("GetSceneLogs", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddSceneActivation", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}