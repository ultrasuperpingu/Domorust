use std::collections::HashMap;

use warp::Filter;


pub(crate) fn users() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_users().or(add_user()).or(update_user()).or(delete_user())
}

fn get_users() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "users")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::users::get_users)
}
fn add_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "users")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::users::add_user)
}
fn update_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "users" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::users::update_user)
}
fn delete_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "users" / usize)
		.and(warp::delete())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::users::delete_user)
}