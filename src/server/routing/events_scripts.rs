use std::collections::HashMap;

use warp::Filter;


pub(crate) fn events_scripts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_events_scripts().or(add_events_scripts()).or(update_events_scripts()).or(delete_events_scripts())
	.or(get_devices_current_status())
}

fn get_events_scripts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "events_scripts")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::events_scripts::get_events_scripts)
}
fn add_events_scripts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "events_scripts")
		.and(warp::post())
		//.and(warp::multipart::form().max_length(5_000_000))
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::events_scripts::add_events_script)
}
fn update_events_scripts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "events_scripts" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::events_scripts::update_events_script)
}
fn delete_events_scripts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "events_scripts" / usize)
		.and(warp::delete())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::events_scripts::delete_events_script)
}
fn get_devices_current_status() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "events_scripts" / "devices_current_status")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::events_scripts::get_devices_current_status)
}