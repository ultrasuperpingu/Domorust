use std::collections::HashMap;

use warp::Filter;


pub(crate) fn settings() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_settings().or(store_settings()).or(get_setting()).or(store_setting())
}

fn get_settings() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "settings")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::settings::get_settings)
}
fn get_setting() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "settings" / String)
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::settings::get_setting)
}
fn store_settings() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "settings")
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and(warp::multipart::form().max_length(500_000))
		.and_then(crate::server::handlers::settings::store_settings)
}
fn store_setting() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "settings" / String)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::settings::set_setting)
}
