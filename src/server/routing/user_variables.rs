use std::collections::HashMap;

use warp::Filter;


pub(crate) fn user_variables() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_user_variables().or(add_user_variable()).or(update_user_variable()).or(delete_user_variable())
}
fn get_user_variables() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "user_variables")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::user_variables::get_user_variables)
}
fn add_user_variable() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "user_variables")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::user_variables::add_user_variable)
}
fn update_user_variable() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "user_variables" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::user_variables::update_user_variable)
}
fn delete_user_variable() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "user_variables" / usize)
		.and(warp::delete())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::user_variables::delete_user_variable)
}
