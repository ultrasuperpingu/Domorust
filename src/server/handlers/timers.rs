use std::{collections::HashMap, convert::Infallible};

use domorust_macros::route;
use domorust_models::timers::{Timer, TimerPlan, TimerType};
use num_traits::FromPrimitive;
use warp::http;
use warp::reply::{self, Reply};

use crate::db;
use crate::server::responses::{RequestError, RequestResult};

#[route(path=("domorust-api" / "timers"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_timers(params: HashMap<String, String>) -> Result<reply::Response, Infallible> {
	let res=db::timers::get_timers(params);
	match res {
		Ok(res) => {
			let res=RequestResult::<Timer>::new("GetTimers", res);
			Ok(reply::json(&res).into_response())
		},
		Err(e) => {
			let res=RequestError::new("GetTimers", e.into());
			Ok(reply::with_status(reply::json(&res), http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "timers" / usize), method="GET", query_params=false, needed_rights=0)]
pub async fn get_timer(idx: usize) -> Result<impl Reply, Infallible> {
	match db::timers::get_timer(idx) {
		Ok(t) => {
			Ok(reply::with_status(reply::json(&RequestResult::<Timer>::new("GetTimer", vec![t])), http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetTimer", e.into())),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "devices" / usize / "timers"), method="GET", query_params=false, needed_rights=0)]
pub async fn get_device_timers(dev_idx:usize) -> Result<impl Reply, Infallible> {
	let res=db::timers::get_device_timers(dev_idx);
	match res {
		Ok(res) => {
			let res=RequestResult::<Timer>::new("GetTimers", res);
			Ok(reply::json(&res).into_response())
		},
		Err(e) => {
			let res=RequestError::new("GetTimers", e);
			Ok(reply::with_status(reply::json(&res), http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

#[route(path=("domorust-api" / "devices" / usize / "timers"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_device_timer(dev_id:usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	let empty=String::new();
	let zero_str="0".to_string();
	let two_str="2".to_string();
	let d128_str="128".to_string();
	let false_str="0".to_string();
	let active_str = params.get("active").unwrap_or(&false_str);
	let command_str = params.get("command").unwrap_or(&zero_str);
	let days_str = params.get("days").unwrap_or(&d128_str);
	let hour_str = params.get("hour").unwrap_or(&zero_str);
	let min_str = params.get("min").unwrap_or(&zero_str);
	let date_str = params.get("date").unwrap_or(&zero_str);
	let level_str = params.get("level").unwrap_or(&zero_str);
	let randomness_str = params.get("randomness").unwrap_or(&false_str);
	let timertype_str = params.get("timertype").unwrap_or(&two_str);
	let _vunit_str = params.get("vunit").unwrap_or(&empty);

	let timer_type = timertype_str.parse::<u8>().unwrap();
	let command = command_str.parse::<u8>().unwrap();
	let level = level_str.parse::<u8>().unwrap();
	let color_str="";
	let randomness = randomness_str.parse::<u8>().unwrap();
	let days = days_str.parse::<u16>().unwrap();
	let month = level_str.parse::<u8>().unwrap();
	let mday = level_str.parse::<u8>().unwrap();
	let occurence = level_str.parse::<u8>().unwrap();
	let timer = Timer{
		Active:active_str.to_lowercase() == "true",
		Date:date_str.to_string(),
		DeviceRowID: dev_id,
		Time:hour_str.to_owned()+":"+min_str,
		Type:FromPrimitive::from_u8(timer_type).unwrap_or(TimerType::OnTime),
		Cmd:command,
		Level:level,
		Color:color_str.to_string(),
		Randomness:randomness == 1,
		Days:days,
		Month:month,
		MDay:mday,
		Occurence:occurence,
		Persistant:false,
		ID:0
	};
	match db::timers::add_timer(dev_id, &timer) {
		Ok(()) => Ok(reply::with_status(reply::json(&RequestResult::<String>::new("UpdateTimer", vec![])),http::StatusCode::OK)),
		Err(e) => Ok(reply::with_status(reply::json(&RequestError::new("UpdateTimer", e.into())),http::StatusCode::INTERNAL_SERVER_ERROR))
	}
}
#[route(path=("domorust-api" / "timers" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_timer(idx: usize,params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	let empty=String::new();
	let zero_str="0".to_string();
	let two_str="2".to_string();
	let d128_str="128".to_string();
	let false_str="0".to_string();
	let active_str = params.get("active").unwrap_or(&false_str);
	let command_str = params.get("command").unwrap_or(&zero_str);
	let days_str = params.get("days").unwrap_or(&d128_str);
	let hour_str = params.get("hour").unwrap_or(&zero_str);
	let min_str = params.get("min").unwrap_or(&zero_str);
	let date_str = params.get("date").unwrap_or(&zero_str);
	let level_str = params.get("level").unwrap_or(&zero_str);
	let randomness_str = params.get("randomness").unwrap_or(&false_str);
	let timertype_str = params.get("timertype").unwrap_or(&two_str);
	let _vunit_str = params.get("vunit").unwrap_or(&empty);

	let timer_type = timertype_str.parse::<u8>().unwrap();
	let command = command_str.parse::<u8>().unwrap();
	let level = level_str.parse::<u8>().unwrap();
	let color_str="";
	let randomness = randomness_str.parse::<u8>().unwrap();
	let days = days_str.parse::<u16>().unwrap();
	let month = level_str.parse::<u8>().unwrap();
	let mday = level_str.parse::<u8>().unwrap();
	let occurence = level_str.parse::<u8>().unwrap();
	let timer = Timer{
		Active:active_str.to_lowercase() == "true",
		Date:date_str.to_string(),
		DeviceRowID: _vunit_str.parse().unwrap(),
		Time:hour_str.to_owned()+":"+min_str,
		Type:FromPrimitive::from_u8(timer_type).unwrap_or(TimerType::OnTime),
		Cmd:command,
		Level:level,
		Color:color_str.to_string(),
		Randomness:randomness == 1,
		Days:days,
		Month:month,
		MDay:mday,
		Occurence:occurence,
		Persistant:false,
		ID:idx
	};
	match db::timers::update_timer(idx, &timer) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("UpdateTimer", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateTimer", e)),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
	
}
#[route(path=("domorust-api" / "timers" / usize), method="DELETE", query_params=false, needed_rights=2)]
pub async fn delete_timer(idx: usize) -> Result<impl Reply, Infallible> {
	match db::timers::delete_timer(idx) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("DeleteTimer", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteTimer", e.into())),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "devices" / usize / "timers"), method="PUT", query_params=false, needed_rights=2)]
pub async fn delete_device_timers(dev_id: usize) -> Result<impl Reply, Infallible> {
	match db::timers::delete_device_timers(dev_id) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("ClearTimer", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("ClearTimer", e.into())),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "timerplans"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_timerplans(filters: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::timers::get_timerplans(filters) {
		Ok(res) => {
			Ok(reply::with_status(reply::json(&RequestResult::<TimerPlan>::new("GetTimerPlan", res)),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetTimerPlan", e.into())),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "timerplans"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_timerplan(params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::timers::add_timerplan(params.get("name").unwrap()) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("AddTimerPlan", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddTimerPlan", e)),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "timerplans" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_timerplan(id: usize,params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::timers::update_timerplan(id, params.get("name").unwrap()) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("UpdateTimerPlan", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateTimerPlan", e)),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "timerplans" / usize), method="DELETE", query_params=false, needed_rights=2)]
pub async fn delete_timerplan(id: usize) -> Result<impl Reply, Infallible> {
	match db::timers::delete_timerplan(id) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("DeleteTimerPlan", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteTimerPlan", e.into())),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
#[route(path=("domorust-api" / "timerplans" / usize / "duplicate"), method="POST", query_params=true, needed_rights=2)]
pub async fn duplicate_timerplan(id: usize, params: HashMap<String, String>) -> Result<impl Reply, Infallible> {
	match db::timers::duplicate_timerplan(id, params.get("name").unwrap()) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<String>::new("DuplicateTimerPlan", vec![])),http::StatusCode::OK))
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DuplicateTimerPlan", e)),http::StatusCode::INTERNAL_SERVER_ERROR))
		}
	}
}
