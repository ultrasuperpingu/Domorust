
use std::collections::HashMap;

use warp::Filter;
use warp_sessions::{CookieOptions, SameSiteCookieOption};

use crate::server::session_store::MySessionStore;

pub(crate) fn login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	//TODO: auth should be done for all domorust-api, not just login...
	let co=Some(CookieOptions {
		cookie_name: "sid",
		cookie_value: None,
		max_age: None,
		domain: None,
		path: Some("/domorust-api/login".to_string()),
		secure: false,
		http_only: false,
		same_site: Some(SameSiteCookieOption::Strict),
	});
	login_check(co.clone()).or(login_query(co.clone())).or(logout(co))
	
}
fn login_check(co : Option<CookieOptions>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "login")
	.and(warp::get())
	.and(warp::addr::remote())
	.and(warp_sessions::request::with_session(
		MySessionStore::new(),
		co,
	))
	.and(warp::query::<HashMap<String, String>>())
	.and_then(crate::server::handlers::login::login_check)
	.untuple_one()
	.and_then(warp_sessions::reply::with_session)
}
fn login_query(co : Option<CookieOptions>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "login")
	.and(warp::post())
	.and(warp::addr::remote())
	.and(warp_sessions::request::with_session(
		MySessionStore::new(),
		co,
	))
	.and(warp::query::<HashMap<String, String>>())
	.and_then(crate::server::handlers::login::login_query)
	.untuple_one()
	.and_then(warp_sessions::reply::with_session)
}
fn logout(co : Option<CookieOptions>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "login")
	.and(warp::delete())
	.and(warp_sessions::request::with_session(
		MySessionStore::new(),
		co,
	))
	.and(warp::query::<HashMap<String, String>>())
	.and_then(crate::server::handlers::login::logout)
	.untuple_one()
	.and_then(warp_sessions::reply::with_session)
}