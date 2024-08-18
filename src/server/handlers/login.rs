use std::net::SocketAddr;
use std::{collections::HashMap, convert::Infallible};

use chrono::{Duration, Utc};
use domorust_models::users::User;
use domorust_models::utils::base64;
use warp::reply::{self, Reply};
use warp::http::StatusCode;
use warp_sessions::SessionWithStore;
use crate::db;
use crate::server::responses::{AuthResponse, RequestError, RequestResult};
use crate::server::session_store::MySessionStore;


pub async fn login_check(_socket:Option<SocketAddr>, store: SessionWithStore<MySessionStore>, _params: HashMap<String, String>) -> Result<(impl warp::reply::Reply, SessionWithStore<MySessionStore>), Infallible> {
	println!("Expire: {}, Username: {}, Rights: {}", store.session.is_expired(), store.session.get::<String>("Username").is_none(), store.session.get::<String>("Rights").is_none());
	println!("login_check session: {:?}", store.session);
	if store.session.is_expired() || store.session.get::<String>("Username").is_none()|| store.session.get::<u16>("Rights").is_none() {
		return Ok((warp::reply::with_status(warp::reply::json(&AuthResponse::unauthorized()), warp::http::StatusCode::UNAUTHORIZED).into_response(), store))
	};
	//TODO: Check remote addresse is the same ???
	let username:String=store.session.get("Username").unwrap();
	let dec=base64::decode(username).unwrap();
	let str=dec.as_slice();
	let decusername=std::str::from_utf8(str).unwrap();
	Ok((warp::reply::json(
		&AuthResponse::authorized(
			decusername.to_string(),
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
		return Ok((reply::with_status(reply::json(&RequestError::new("Login","".into())), StatusCode::BAD_REQUEST).into_response(), store))
	}
	match db::users::get_users(&params.clone()) {
		Ok(res) => {
			//Ok(reply::json(&RequestResult::new("GetScenes".to_string(), res)).into_response())
			let username=params.get("username").unwrap();
			let dec=base64::decode(username).unwrap();
			let str=dec.as_slice();
			let decusername=std::str::from_utf8(str).unwrap();
			let u:Vec<&User>=res.iter().filter(|u| &u.Username == username).collect();
			if u.len() > 1 {
				return Ok((reply::with_status(reply::json(&RequestError::new("Login","".into())), StatusCode::INTERNAL_SERVER_ERROR).into_response(), store))
			}
			else if u.is_empty() {
				return Ok((reply::with_status(reply::json(&RequestError::new("Login","".into())), StatusCode::UNAUTHORIZED).into_response(), store))
			}
			//TODO: handle Active
			if &u[0].Password == params.get("password").unwrap() {
				let _ = store.session.insert("Username", username);
				let id= store.session.id().to_string();
				let _ = store.session.insert("AuthToken", id);
				let _ = store.session.insert("RemoteHost", remote);
				let _ = store.session.insert("Rights", u[0].Rights);
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
				return Ok((reply::with_status(reply::json(&AuthResponse::authorized(decusername.to_string(),u[0].Rights)), StatusCode::OK).into_response(), store))
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