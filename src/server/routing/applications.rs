use std::collections::HashMap;

//use warp_sessions::{MemoryStore, SessionWithStore, CookieOptions, SameSiteCookieOption};

use warp::Filter;


pub(crate) fn applications() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_applications().or(add_application()).or(update_application()).or(delete_application())
}

fn get_applications() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "applications")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::applications::get_applications)
}
fn add_application() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "applications")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::applications::add_application)
}
fn update_application() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "applications" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::applications::update_application)
}
fn delete_application() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "applications" / usize)
		.and(warp::delete())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::applications::delete_application)
}