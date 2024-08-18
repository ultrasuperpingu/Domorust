use std::collections::HashMap;

use warp::Filter;


pub(crate) fn hardwares() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_hardwares().or(get_hardware()).or(add_hardware()).or(update_hardware()).or(delete_hardware())
}

fn get_hardwares() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "hardwares")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::hardwares::get_hardwares)
}
fn get_hardware() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "hardwares" / usize)
		.and(warp::get())
		.and_then(crate::server::handlers::hardwares::get_hardware)
}
fn add_hardware() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "hardwares")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::hardwares::add_hardware)
}
fn update_hardware() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "hardwares" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::hardwares::update_hardware)
}
fn delete_hardware() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "hardwares" / usize)
		.and(warp::delete())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::hardwares::delete_hardware)
}

