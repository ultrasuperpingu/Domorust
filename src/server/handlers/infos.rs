use std::convert::Infallible;

use domorust_macros::route;
use domorust_models::hardware::get_hardware_types_data;
use domorust_models::settings::ConfigResponseSettings;
use domorust_models::UptimeData;
use warp::reply::{self, Reply, Response};
use warp::http::StatusCode;

use crate::server::responses::{get_serial_ports, RequestSingleResult};
use crate::server::responses::{ RequestError, RequestResult };


#[route(path=("domorust-api" / "infos" / "version"), method="GET", query_params=false, needed_rights=-1)]
pub async fn get_version()  -> Result<impl Reply, Infallible> {
	Ok(reply::json(
		&RequestResult::<String>::new(
			"GetVersion",
			vec!["0.1".to_string()]
		))
	)
}
#[route(path=("domorust-api" / "infos" / "uptime"), method="GET", query_params=false, needed_rights=-1)]
pub async fn get_uptime()  -> Result<impl Reply, Infallible> {
	Ok(reply::json(
		&RequestResult::<UptimeData>::new(
			"GetUptime",
			vec![UptimeData{days:12,hours:1,minutes:21,seconds:11}]
		))
	)
}
#[route(path=("domorust-api" / "infos" / "sun_rise_set"), method="GET", query_params=false, needed_rights=-1)]
pub async fn get_sun_rise_set()  -> Result<Response, Infallible> {
	let sunrise_set = crate::server::responses::get_sun_rise_set();
	match sunrise_set {
		Ok(sunrise_set) => {
			Ok(reply::json(&sunrise_set).into_response())
		},
		Err(e) => {
			let res = RequestError::new("GetSunRiseSet", e);
			Ok(reply::with_status(reply::json(&res), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "infos" / "serial_devices"), method="GET", query_params=false, needed_rights=0)]
pub async fn get_serial_devices()  -> Result<Response, Infallible> {
	let htds=get_serial_ports();
	match htds {
		Ok(htds) => {
			let res = RequestResult::new("GetHardwareTypes", htds);
			Ok(reply::json(&res).into_response())
		},
		Err(e) => {
			let res = RequestError::new("GetHardwareTypes", e);
			Ok(reply::with_status(reply::json(&res), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "infos" / "hardwaretypes"), method="GET", query_params=false, needed_rights=0)]
pub async fn get_hardwaretypes()  -> Result<Response, Infallible> {
	let htds=get_hardware_types_data();
	match htds {
		Ok(htds) => {
			let res = RequestResult::new("GetHardwareTypes", htds);
			Ok(reply::json(&res).into_response())
		},
		Err(e) => {
			let res = RequestError::new("GetHardwareTypes", e);
			Ok(reply::with_status(reply::json(&res), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "infos" / "switchtypes"), method="GET", query_params=false, needed_rights=-1)]
pub async fn get_switchtypes()  -> Result<impl Reply, Infallible> {
	let res = crate::server::responses::get_switch_types();
	Ok(reply::json(&res))
}
#[route(path=("domorust-api" / "infos" / "metertypes"), method="GET", query_params=false, needed_rights=-1)]
pub async fn get_metertypes()  -> Result<impl Reply, Infallible> {
	let res = crate::server::responses::get_meter_types();
	Ok(reply::json(&res))
}
#[route(path=("domorust-api" / "infos" / "languages"), method="GET", query_params=false, needed_rights=-1)]
pub async fn get_languages()  -> Result<impl Reply, Infallible> {
	let res = crate::server::responses::get_languages();
	Ok(reply::json(&res))
}
#[route(path=("domorust-api" / "infos" / "config"), method="GET", query_params=false, needed_rights=0)]
pub async fn get_config()  -> Result<impl Reply, Infallible> {
	println!("get_config");
	match crate::db::get_config() {
		Ok(mut res) => {
			complete_config_resp(&mut res);
			Ok(reply::json(&RequestSingleResult::new("GetConfig",res)).into_response())
		},
		Err(e) => Ok(reply::with_status(reply::json(&RequestError::new("title", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
	}
}
fn complete_config_resp(res:&mut ConfigResponseSettings) {
	//TODO, get current user with session...
	//let params = HashMap::new();
	//get_users(params);
	let wind_val=res.WindUnit.value();
	res.WindScale = wind_val.1;
	res.WindSign = wind_val.0.to_string();
	
	let weight_val=res.WeightUnit.value();
	res.WeightScale = weight_val.1;
	res.WeightSign = weight_val.0.to_string();
	
	let temp_val=res.TempUnit.value();
	res.TempScale = temp_val.1;
	res.TempSign = temp_val.0.to_string();
	
	res.result.EnableTabCustom = true;
	res.result.EnableTabDashboard = true;
	res.result.EnableTabFloorplans = true;
	res.result.EnableTabLights = true;
	res.result.EnableTabScenes = true;
	res.result.EnableTabTemp = true;
	res.result.EnableTabUtility = true;
	res.result.EnableTabWeather = true;
}