use std::collections::HashMap;

use warp::Filter;


pub(crate) fn devices() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_devices().or(get_light_switches()).or(get_device()).or(add_device()).or(update_device()).or(delete_device())
	.or(get_device_graph()).or(allow_new_devices()).or(make_favorite_device())
	.or(send_device_command())
}

fn get_devices() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::get_devices)
}
fn get_light_switches() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / "light_switches")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::get_light_switches)
}
fn get_device() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize)
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::get_device)
}
fn add_device() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::add_device)
}
fn update_device() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::update_device)
}
fn delete_device() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize)
		.and(warp::delete())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::delete_device)
}

fn get_device_graph() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize / "graph")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::get_device_graph)
}
fn allow_new_devices() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / "allow_new")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::allow_new_devices)
}
fn make_favorite_device() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize / "make_favorite")
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::make_favorite_device)
}
fn send_device_command() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "devices" / usize / "commands" / String)
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::devices::send_device_command)
}
//https://localhost:2443/domorust-api?type=command&param=setcolbrightnessvalue&idx=44&color={"m":2,"t":63,"r":0,"g":0,"b":0,"cw":192,"ww":63}&brightness=NaN
//https://localhost:2443/domorust-api?type=command&param=switchlight&idx=44&switchcmd=On&level=0&passcode=