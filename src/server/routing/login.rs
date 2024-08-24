
use std::collections::HashMap;

use warp::Filter;
use warp_sessions::CookieOptions;

use crate::server::session_store::MySessionStore;

use crate::server::session_store::COOKIE;
use crate::balanced_or_tree;
use crate::debug_boxed;

pub(crate) fn login() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	balanced_or_tree!(
		login_check(COOKIE.clone()),
		login_query(COOKIE.clone()),
		logout(COOKIE.clone())
	)
}
pub(crate) fn login_check(co : Option<CookieOptions>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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
pub(crate) fn login_query(co : Option<CookieOptions>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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
pub(crate) fn logout(co : Option<CookieOptions>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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