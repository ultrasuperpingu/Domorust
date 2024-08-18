use std::collections::HashMap;

use warp::Filter;


pub(crate) fn plans() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_floorplans().or(get_floorplan()).or(get_floorplan_image())
	.or(get_plans()).or(get_plan()).or(add_plan()).or(update_plan()).or(delete_plan())
	.or(get_plan_devices())
}
fn get_floorplans() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "floorplans")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::get_floorplans)
}
fn get_floorplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "floorplans" / usize)
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::get_floorplan)
}
fn get_floorplan_image() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "floorplans" / usize / "image")
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::get_floorplan_image)
}
fn get_plans() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "plans")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::get_plans)
}
fn get_plan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "plans" / usize)
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::get_plan)
}
fn add_plan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "floorplans" / usize / "plans")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::add_plan)
}
fn update_plan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "plans" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::update_plan)
}
fn delete_plan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "plans" / usize)
		.and(warp::put())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::delete_plan)
}
fn get_plan_devices() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "plans" / usize / "devices")
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::plans::get_plan_devices)
}