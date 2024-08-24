use std::{collections::HashMap, convert::Infallible};

use chrono::Utc;
use domorust_macros::route;
use domorust_models::users::User;
use sha2::{Digest, Sha256};
use warp::reply::{self, Reply};
use warp::http::StatusCode;

use crate::db;
use crate::server::responses::{RequestError, RequestResult};

use super::login::PEPPER;

#[route(path=("domorust-api" / "users"), method="GET", query_params=true, needed_rights=0)]
pub async fn get_users(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::users::get_users(&params) {
		Ok(res) => {
			let req_res=RequestResult::new("GetUsers", res);
			let resp=reply::json(&req_res).into_response();
			Ok(resp)
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetUsers", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "users" / usize), method="GET", query_params=false, needed_rights=0)]
pub async fn get_user(id: usize) -> Result<impl warp::Reply, Infallible> {
	match db::users::get_user(id) {
		Ok(res) => {
			let req_res=RequestResult::new("GetUsers", vec![res]);
			let resp=reply::json(&req_res).into_response();
			Ok(resp)
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetUsers", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "users"), method="POST", query_params=true, needed_rights=2)]
pub async fn add_user(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	//TODO: remove unwrap (move code to db module)
	let mut params = params.clone();
	match db::users::get_users(&params) {
		Ok(res) => {
			let username=params.get("username").unwrap();
			//let dec=base64::decode(username).unwrap();
			//let str=dec.as_slice();
			//let decusername=std::str::from_utf8(str).unwrap();
			let u:Vec<&User>=res.iter().filter(|u| &u.Username == username).collect();
			if !u.is_empty() {
				return Ok(reply::with_status(reply::json(&RequestError::new("AddUser","User already exists".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response())
			}
			let salt=format!("{:X}",Sha256::digest(Utc::now().naive_local().to_string()+username.as_str()));
			let mut hasher = Sha256::new();
			hasher.update(salt.clone() + params.get("password").unwrap() + PEPPER.as_str());
			let pwhash = format!("{:X}", hasher.finalize());
			params.insert("password".to_string(), pwhash);
			params.insert("Salt".to_string(), salt);
		},
		Err(e) => {
			return Ok(reply::with_status(reply::json(&RequestError::new("AddUser",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
	match db::users::add_user(&params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("AddUser", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddUser", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "users" / usize), method="PUT", query_params=true, needed_rights=2)]
pub async fn update_user(idx: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	let username=params.get("username").unwrap();
	let password = params.get("password").unwrap();
	//let encusername=base64::encode(username);
	let mut params2 = HashMap::new();
	params2.insert("username".to_string(), username.clone());
	match db::users::get_users(&params2) {
		Ok(res) => {
			let u:Vec<&User>=res.iter().filter(|u| {
				println!("{} {}", u.Username, username);
				&u.Username == username
			}).collect();
			if u.is_empty() {
				return Ok(reply::with_status(reply::json(&RequestError::new("UpdateUser","No User found".into())), StatusCode::NOT_FOUND).into_response())
			}
			let mut hasher = Sha256::new();
			hasher.update(u[0].Salt.clone() + password + PEPPER.as_str());
			let pwhash = format!("{:X}", hasher.finalize());
			params2.insert("password".to_string(), pwhash);
		},
		Err(e) => {
			return Ok(reply::with_status(reply::json(&RequestError::new("UpdateUser",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
	let mut params3=params.clone();
	params3.extend(params2);
	match db::users::update_user(idx, &params3) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("UpdateUser", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateUser", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
#[route(path=("domorust-api" / "users" / usize), method="DELETE", query_params=true, needed_rights=2)]
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