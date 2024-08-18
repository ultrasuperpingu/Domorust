use std::{collections::HashMap, convert::Infallible};

use warp::reply::{self, Reply};
use warp::http::StatusCode;

use crate::db;
use crate::server::responses::{RequestError, RequestResult};


pub async fn get_users(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::users::get_users(&params) {
		Ok(res) => {
			Ok(reply::json(&RequestResult::new("GetUsers", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetUsers",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}}

pub async fn add_user(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::users::add_user(&params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("AddUser", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddUser", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn update_user(idx: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::users::update_user(idx, &params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("UpdateUser", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateUser", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn delete_user(idx: usize, _params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::users::delete_user(idx) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("DeleteUser", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteUser", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}