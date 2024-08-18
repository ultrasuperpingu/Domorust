use std::collections::HashMap;

use domorust_models::domorust::Domorust;
use warp::Filter;

use super::with_domorust;


pub(crate) fn themes(domorust: &Domorust) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_themes(domorust).or(get_theme()).or(set_theme())
}

fn get_themes(domorust: &Domorust) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "themes")
		.and(warp::get())
		//TODO: remove clone...
		.and(with_domorust(domorust.clone()))
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::themes::get_themes)
}
fn get_theme() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "themes" / "current")
		.and(warp::get())
		.and_then(crate::server::handlers::themes::get_theme)
}
fn set_theme() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "themes" / "current")
		.and(warp::put())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(crate::server::handlers::themes::set_theme)
}
