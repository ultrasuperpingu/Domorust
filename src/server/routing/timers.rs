use std::collections::HashMap;

use warp::Filter;


pub(crate) fn timers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_timers().or(get_timer()).or(get_device_timers())
	.or(add_device_timer()).or(update_timer())
	.or(delete_timer()).or(delete_device_timers())
	.or(get_timerplans()).or(add_timerplan()).or( update_timerplan())
	.or(delete_timerplans()).or(duplication_timerplan())
}

fn get_timers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timers")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::get_timers)
}
fn get_timer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timers" / usize)
		.and(warp::get())
		.and_then(crate::server::handlers::timers::get_timer)
}
fn get_device_timers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize / "timers")
		.and(warp::get())
		.and_then(crate::server::handlers::timers::get_device_timers)
}
fn add_device_timer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize / "timers")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::add_timer)
}
fn update_timer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timers" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::update_timer)
}
fn delete_timer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timers" / usize)
		.and(warp::delete())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::delete_timer)
}
fn delete_device_timers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize / "timers")
		.and(warp::delete())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::delete_device_timers)
}
fn get_timerplans() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timerplans" )
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::get_timerplans)
}
fn add_timerplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timerplans")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::add_timerplan)
}
fn update_timerplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timerplans" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::update_timerplan)
}
fn delete_timerplans() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timerplans" / usize)
		.and(warp::delete())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::delete_timerplan)
}
fn duplication_timerplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "timerplans" / usize / "duplicate")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::timers::duplicate_timerplan)
}