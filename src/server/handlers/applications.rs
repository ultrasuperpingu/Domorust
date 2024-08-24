use std::{collections::HashMap, convert::Infallible};

use domorust_macros::route;
use warp::reply::{self, Reply, Response};
use warp::http::StatusCode;
use crate::db;

use crate::server::responses::{RequestError, RequestResult};

#[route(path=("domorust-api" / "applications"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_applications(params: HashMap<String, String>) -> Result<Response, Infallible> {
	match db::applications::get_applications(params) {
		Ok(res) => {
			Ok(reply::with_status(reply::json(&RequestResult::new("GetApplications",res)), StatusCode::OK).into_response())
		},
		Err(e) => {
			eprintln!("Error on get_application: {}", e);
			Ok(reply::with_status(reply::json(&RequestError::new("GetApplications", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "applications"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_application(params: HashMap<String, String>) -> Result<Response, Infallible> {
	//type=command&param=addapplication&idx=0&enabled=true&applicationname=toto&secret=&pemfile=&public=
	match db::applications::add_application(params) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<()>::new("AddApplication",vec![])), StatusCode::OK).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddApplication",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "applications" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_application(idx: usize, params: HashMap<String, String>) -> Result<Response, Infallible> {
	//type=command&param=updateapplication&idx=0&enabled=true&applicationname=toto&secret=&pemfile=&public=
	match db::applications::update_application(idx, params) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<()>::new("UpdateApplication",vec![])), StatusCode::OK).into_response())
		}
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateApplication",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

#[route(path=("domorust-api" / "applications" / usize), method="DELETE", query_params=true, needed_rights=2)]
pub async fn delete_application(idx: usize, _params: HashMap<String, String>) -> Result<Response, Infallible> {
	//let query=format!("DELETE FROM Application WHERE ID == {}", idx);
	match db::applications::delete_application(idx) {
		Ok(()) => {
			Ok(reply::with_status(reply::json(&RequestResult::<()>::new("DeleteApplication",vec![])), StatusCode::OK).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteApplication",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}