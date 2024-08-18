use std::collections::HashMap;

use warp::Filter;


pub(crate) fn scenes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_scenes().or(add_scene()).or(update_scene()).or(delete_scene())
	.or(get_scene_activations()).or(get_scene_devices()).or(get_scene_logs()).or(get_scene_timers())
	.or(add_scene_activation()).or(add_scene_device()).or(add_scene_timer())
}

fn get_scenes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::get_scenes)
}
fn add_scene() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::add_scene)
}
fn update_scene() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::update_scene)
}
fn delete_scene() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize)
		.and(warp::delete())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::delete_scene)
}
fn get_scene_devices() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "devices")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::get_scene_devices)
}
fn get_scene_activations() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "activations")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::get_scene_activations)
}
fn get_scene_timers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "timers")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::get_scene_timers)
}
fn get_scene_logs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "logs")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::get_scene_logs)
}
fn add_scene_device() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "devices")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::add_scene_device)
}
fn add_scene_activation() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "activations")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::add_scene_activation)
}
fn add_scene_timer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "scenes" / usize / "timers")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::scenes::add_scene_timer)
}