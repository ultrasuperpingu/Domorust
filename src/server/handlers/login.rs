use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::LazyLock;
use std::{collections::HashMap, convert::Infallible};

use chrono::{Duration, Utc};
use domorust_models::users::User;
use sha2::{Digest, Sha256};
use warp::reply::{self, Reply};
use warp::http::StatusCode;
use warp_sessions::SessionWithStore;
use crate::db;
use crate::server::responses::{AuthResponse, RequestError, RequestResult};
use crate::server::session_store::MySessionStore;

pub(crate) static PEPPER : LazyLock<String> = LazyLock::new(|| {
	match std::fs::read_to_string("pepper.txt") {
		Ok(pepper) => pepper,
		Err(_e) => {
			let mut file = File::create("pepper.txt").unwrap();
			//TODO: better way to generate pepper
			let mut pepper_str = Utc::now().to_string();
			let mut hasher = Sha256::new();
			hasher.update(pepper_str);
			pepper_str = format!("{:X}", hasher.finalize());
			
			file.write_all(pepper_str.as_bytes()).unwrap();
			pepper_str
		},
	}
});

pub async fn login_check(_socket:Option<SocketAddr>, store: SessionWithStore<MySessionStore>, _params: HashMap<String, String>) -> Result<(impl warp::reply::Reply, SessionWithStore<MySessionStore>), Infallible> {
	println!("Expire: {}, Username: {:?}, Rights: {:?}", store.session.is_expired(), store.session.get::<String>("Username"), store.session.get::<String>("Rights"));
	println!("login_check session: {:?}", store.session);
	if store.session.is_expired() || store.session.get::<String>("Username").is_none()|| store.session.get::<u16>("Rights").is_none() {
		return Ok((warp::reply::with_status(warp::reply::json(&AuthResponse::unauthorized()), warp::http::StatusCode::UNAUTHORIZED).into_response(), store))
	};
	//TODO: Check remote addresse is the same ???
	let username:String=store.session.get("Username").unwrap();
	Ok((warp::reply::json(
		&AuthResponse::authorized(
			username.to_string(),
			store.session.get::<u16>("Rights").unwrap()
		)
	).into_response(), store))
	//Ok(reply::with_status(reply::json(&AuthResponse::default()), StatusCode::OK).into_response())
}

pub async fn login_query(socket:Option<SocketAddr>, mut store: SessionWithStore<MySessionStore>, params: HashMap<String, String>) -> Result<(impl warp::reply::Reply, SessionWithStore<MySessionStore>), Infallible> {
	let remote=if let Some(add)=socket {
		add.ip().to_string()
	} else {
		String::new()
	};
	if !params.contains_key("username") || !params.contains_key("password") {
		return Ok((reply::with_status(reply::json(&RequestError::new("Login","Need params username and password".into())), StatusCode::BAD_REQUEST).into_response(), store))
	}
	let password = params.get("password").unwrap();
	let mut params = params.clone();
	params.remove("password");
	match db::users::get_users(&params.clone()) {
		Ok(res) => {
			if !res.iter().any(|u| u.Active) {
				let _= store.session.insert("NoUsers", true);
				return Ok((reply::with_status(reply::json(&AuthResponse::no_users()), StatusCode::OK).into_response(), store))
			}
			let username=params.get("username").unwrap();
			let u:Vec<&User>=res.iter().filter(|u| &u.Username == username).collect();
			if u.len() > 1 {
				return Ok((reply::with_status(reply::json(&RequestError::new("Login","".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response(), store))
			}
			else if u.is_empty() {
				return Ok((reply::with_status(reply::json(&RequestError::new("Login","".into())), StatusCode::UNAUTHORIZED).into_response(), store))
			}
			let mut hasher = Sha256::new();
			hasher.update(u[0].Salt.clone() + password + PEPPER.as_str());
			let pwhash = format!("{:X}", hasher.finalize());
			
			//TODO: handle Active
			if u[0].Password == pwhash {
				let _ = store.session.insert("Username", username);
				let id= store.session.id().to_string();
				let _ = store.session.insert("AuthToken", id);
				let _ = store.session.insert("RemoteHost", remote);
				let _ = store.session.insert("Rights", u[0].Rights);
				//let _ = store.session.insert("Salt", u[0].Salt.clone());
				let rememberme=params.get("rememberme");
				let rememberme=rememberme.is_some() && rememberme.unwrap() == "true";
				if rememberme {
					let _ = store.session.insert("ExpirationDate", "");
					store.session.clear_expiry();
				} else {
					let expiry_date = Utc::now() + Duration::days(2);
					let _ = store.session.insert("ExpirationDate", expiry_date.naive_local().to_string());
					store.session.set_expiry(expiry_date);
				}
				return Ok((reply::with_status(reply::json(&AuthResponse::authorized(username.to_string(),u[0].Rights)), StatusCode::OK).into_response(), store))
			}
		},
		Err(e) => {
			return Ok((reply::with_status(reply::json(&RequestError::new("Login",e)), StatusCode::UNAUTHORIZED).into_response(), store))
		}
	}
	Ok((reply::with_status(reply::json(&RequestError::new("Login","".into())), StatusCode::UNAUTHORIZED).into_response(), store))
}

pub async fn logout(mut store: SessionWithStore<MySessionStore>, _params: HashMap<String, String>) -> Result<(impl warp::reply::Reply, SessionWithStore<MySessionStore>), Infallible> {
	store.session.destroy();
	Ok((reply::with_status(reply::json(&RequestResult::new("Login","".into())), StatusCode::OK).into_response(), store))
}

pub async fn test_auth(store:&SessionWithStore<MySessionStore>, addr:Option<SocketAddr>, needed_rights:i32) -> Result<(), impl warp::reply::Reply> {
	if store.session.is_expired() ||
		store.session.get::<String>("Username").is_none() ||
		store.session.get::<u16>("Rights").is_none() ||
		store.session.get::<String>("RemoteHost").is_none()
	{
		return Err(warp::reply::with_status(warp::reply::json(&AuthResponse::unauthorized()), warp::http::StatusCode::UNAUTHORIZED).into_response())
	}
	let user_rights=store.session.get::<i32>("Rights").unwrap();
	if user_rights < needed_rights {
		return Err(warp::reply::with_status(warp::reply::json(&AuthResponse::unauthorized()), warp::http::StatusCode::UNAUTHORIZED).into_response())
	}
	let remote=store.session.get::<String>("RemoteHost").unwrap();
	if addr.is_some() && remote != addr.unwrap().ip().to_string() {
		//return Ok((warp::reply::with_status(warp::reply::json(&AuthResponse::unauthorized()), warp::http::StatusCode::UNAUTHORIZED).into_response(), store))
	}
	Ok(())
}
