use std::collections::HashMap;
use std::convert::Infallible;

use domorust_macros::route;
use domorust_models::hardware::IHardware;
use domorust_models::plugins::py_domoticz::{get_pydevices, PyHardware};
use warp::reply::{self, Reply, Response};
use warp::http::StatusCode;

use crate::db;
use crate::server::responses::{RequestError, RequestResult};

#[route(path=("domorust-api" / "devices"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_devices(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::devices::get_devices(params) {
		Ok(devices) => {
			let req=RequestResult::new("Devices", devices);
			//println!("{:?}", req);
			//let serde_serialize=serde_json::to_string(&req);
			//println!("{:?}", serde_serialize);
			let json = reply::json(&req).into_response();
			//println!("{:?}", json);
			Ok(json)
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Devices", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "devices" / usize), method="GET", query_params=true, needed_rights=0)]
pub async fn get_device(idx:usize, params: HashMap<String, String>) -> Result<Response, Infallible> {
	let mut params = params.clone();
	params.insert("idx".into(), idx.to_string());
	match db::devices::get_devices(params) {
		Ok(devices) => {
			Ok(reply::json(&RequestResult::new("Devices", devices)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Devices", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "devices" / "light_switches"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_light_switches(params: HashMap<String, String>) -> Result<Response, Infallible> {
	match db::devices::get_light_switches(params) {
		Ok(devices) => {
			Ok(reply::json(&RequestResult::new("GetLightSwitches", devices)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetLightSwitches", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "devices"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_device(params: HashMap<String, String>) -> Result<Response, Infallible> {
	println!("add_device {:?}", params);
	Ok(reply::with_status(reply::json(&RequestError::new("AddDevice","Not implemented".into())), StatusCode::NOT_IMPLEMENTED).into_response())
}
#[route(path=("domorust-api" / "devices" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_device(idx: usize, params: HashMap<String, String>) -> Result<Response, Infallible> {
	println!("update_device {} {:?}", idx, params);
	Ok(reply::with_status(reply::json(&RequestError::new("UpdateDevice","Not implemented".into())), StatusCode::NOT_IMPLEMENTED).into_response())
}
#[route(path=("domorust-api" / "devices" / usize), method="DELETE", query_params=true, needed_rights=2)]
pub async fn delete_device(idx: usize,params: HashMap<String, String>) -> Result<Response, Infallible> {
	println!("delete_device {} {:?}", idx, params);
	Ok(reply::with_status(reply::json(&RequestError::new("DeleteDevice","Not implemented".into())), StatusCode::NOT_IMPLEMENTED).into_response())
}
#[route(path=("domorust-api" / "devices" / usize / "graph"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_device_graph(idx: usize,params: HashMap<String, String>) -> Result<Response, Infallible> {
	//idx=1&param=graph&range=year&sensor=temp&type=command
	//idx=1&param=graph&range=day&sensor=temp&type=command
	//groupby=month&idx=1&param=graph&range=compare&sensor=temp&type=command&var_name=Temp_Avg
	let empty=String::new();
	let res_data=crate::db::get_graph_data(idx, params.get("sensor").unwrap_or(&empty),params.get("range").unwrap_or(&empty));
	match res_data {
		Ok(res_data) => {
			let res=RequestResult::new("Graph",res_data);
			Ok(warp::reply::json(&res).into_response())
		},
		Err(e) => {
			let res=RequestError::new("Graph", Box::new(e));
			Ok(reply::with_status(reply::json(&res), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		},
	}
}
#[route(path=("domorust-api" / "devices" / "allow_new"), method="POST", query_params=true, needed_rights=2)]
pub async fn allow_new_devices(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	println!("allow_new_devices {:?}", params);
	let five="5".to_string();
	let timeout_str = params.get("timeout").unwrap_or(&five);
	if !timeout_str.is_empty() {
		if let Ok(_timeout) = timeout_str.parse::<u16>() {
			//TODO
		}
	}
	Ok(reply::with_status(reply::json(&RequestError::new("AllowNewDevice","Not implemented".into())), StatusCode::NOT_IMPLEMENTED))
}
#[route(path=("domorust-api" / "devices" / usize /"make_favorite"), method="PUT", query_params=true, needed_rights=2)]
pub async fn make_favorite_device(id: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	let favorite = if let Some(favorite) = params.get("favorite") {
		favorite == "true" || favorite == "1"
	} else {
		return Ok(reply::with_status(reply::json(&RequestError::new("Devices", "Missing parameter `favorite`".into())), warp::http::StatusCode::BAD_REQUEST).into_response());
	};
	match db::devices::make_favorite_device(id, favorite) {
		Ok(()) => {
			Ok(reply::json(&RequestResult::<String>::new("Devices", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Devices", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
	//Ok(reply::with_status(reply::json(&RequestError::new("MakeFavoriteDevice".to_string(),"Not implemented".into())), StatusCode::INTERNAL_SERVER_ERROR))
}
// TODO: it's WIP. Just a POC/Trash code. Everything need to be implemented properly
#[route(path=("domorust-api" / "devices" / usize / "commands" / String), method="POST", query_params=true, needed_rights=2)]
pub async fn send_device_command(id: usize, command: String, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	println!("send_device_command device ID {}: {}({:?})", id, command, params);
	let device = crate::db::devices::get_device(id, params).unwrap();
	let hardware = crate::db::hardwares::get_hardware_data(device.HardwareID);
	let domorust=crate::DOMORUST.get().unwrap();
	if let Ok(hardware) = hardware {
		let val = domorust.hardware_types.iter().find(|h| {
			//TODO: Is it the better way ???
			eprintln!("{} {}", (*h).clone().unwrap_python().data.key, hardware.Extra);
			(*h).clone().unwrap_python().data.key == hardware.Extra
		});
		if let Some(val) = val {
			let pyht=val.as_python_ref().unwrap();
			let devices = get_pydevices(db::devices::get_hardware_devices(hardware.ID, HashMap::new()).unwrap());
			let mut ph=PyHardware{data:hardware, module:pyht.module.as_ref(), devices};
			match ph.on_command(device.Unit.to_string(), command, "0".to_string(), "".to_string()) {
				Ok(()) => {
					return Ok(reply::with_status(reply::json(&RequestResult::<String>::new("SendDeviceCommand",vec![])), StatusCode::OK))
				},
				Err(e) => {
					return Ok(reply::with_status(reply::json(&RequestError::new("SendDeviceCommand",e)), StatusCode::INTERNAL_SERVER_ERROR))
				}
			}
		}
	}
	Ok(reply::with_status(reply::json(&RequestError::new("SendDeviceCommand","Not found".into())), StatusCode::NOT_FOUND))
}
