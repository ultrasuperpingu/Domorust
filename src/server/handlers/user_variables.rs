use std::{collections::HashMap, convert::Infallible};
use warp::reply::{self, Reply, Response};
use warp::http::StatusCode;

use crate::server::responses::{RequestError, RequestResult};
use crate::db;


pub async fn get_user_variables(params: HashMap<String, String>) -> Result<Response, Infallible> {
	match db::user_variables::get_user_variables(params) {
		Ok(res) => {
			Ok(reply::json(&RequestResult::new("GetUserVariables", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetUserVariables",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn add_user_variable(params: HashMap<String, String>) -> Result<Response, Infallible> {
	match db::user_variables::add_user_variable(params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("AddUserVariable", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddUserVariable", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn update_user_variable(idx: usize, params: HashMap<String, String>) -> Result<Response, Infallible> {
	match db::user_variables::update_user_variable(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("UpdateUserVariable", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateUserVariable", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn delete_user_variable(idx: usize) -> Result<Response, Infallible> {
	match db::user_variables::delete_user_variable(idx) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("DeleteUserVariable", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteUserVariable", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
