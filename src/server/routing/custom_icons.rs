use std::collections::HashMap;

use warp::Filter;


pub(crate) fn custom_icons() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_custom_icons().or(add_custom_icon()).or(update_custom_icon()).or(delete_custom_icon())
	.or(get_custom_icon_small_image()).or(get_custom_icon_on_image()).or(get_custom_icon_off_image())
	.or(get_images())
}

fn get_custom_icons() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons")
		.and(warp::get())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::get_custom_icons)
}
fn get_custom_icon_small_image() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons" / usize / "image_small")
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::get_custom_icon_small_image)
}
fn get_custom_icon_on_image() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons" / usize / "image_on")
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::get_custom_icon_on_image)
}
fn get_custom_icon_off_image() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons" / usize / "image_off")
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::get_custom_icon_off_image)
}
fn add_custom_icon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons")
		.and(warp::post())
		.and(warp::multipart::form().max_length(5_000_000))
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::add_custom_icon)
}
fn update_custom_icon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons" / usize)
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::update_custom_icon)
}
fn delete_custom_icon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons" / usize)
		.and(warp::delete())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::delete_custom_icon)
}

fn get_images() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "custom_icons" / "images")
		.and(warp::get())
		//.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::custom_icons::get_images)
}