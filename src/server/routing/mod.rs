pub(crate) mod applications;
pub(crate) mod custom_icons;
pub(crate) mod devices;
pub(crate) mod events_scripts;
pub(crate) mod infos;
pub(crate) mod hardwares;
pub(crate) mod login;
pub(crate) mod plans;
pub(crate) mod scenes;
pub(crate) mod settings;
pub(crate) mod themes;
pub(crate) mod timers;
pub(crate) mod user_variables;
pub(crate) mod users;
pub(crate) mod websocket;

use std::{collections::HashMap, convert::Infallible};
use domorust_models::domorust::Domorust;
use warp::{self, Filter};

use crate::handlers;
fn with_domorust(
	domorust: Domorust
) -> impl Filter<Extract = (Domorust,), Error = Infallible> + Clone {
	warp::any().map(move || domorust.clone())
}


/// All customer routes
pub fn routes(domorust: &Domorust) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let wwwroot=domorust.cli.wwwroot.clone();
	domorust_api(domorust)
	.or(domorust_api_old())
	.or(websocket::websocket())
	// TODO: ensure db is not writting
	.or(warp::path("domorust.db").and(warp::get()).and(warp::fs::file("domorust.db")))
	// return the corresponding file in webcontent dir
	.or(warp::fs::dir(wwwroot))
}

fn domorust_api_old() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let get = warp::path("domorust-api")
		.and(warp::get())
		.and(warp::path::full())
		.and(warp::query::<HashMap<String, String>>())
		.and_then(handlers::json_htm_get);
	let post = warp::path("domorust-api")
		.and(warp::post())
		.and(warp::query::<HashMap<String, String>>())
		.and(warp::multipart::form().max_length(500_000))
		.and_then(handlers::json_htm_post);
	get.or(post)
}

fn domorust_api(domorust: &Domorust) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	crate::server::routing::applications::applications()
	.or(crate::server::routing::login::login())
	.or(crate::server::routing::custom_icons::custom_icons())
	.or(crate::server::routing::devices::devices())
	.or(crate::server::routing::hardwares::hardwares())
	.or(crate::server::routing::infos::infos())
	.or(crate::server::routing::scenes::scenes())
	.or(crate::server::routing::timers::timers())
	.or(crate::server::routing::themes::themes(domorust))
	.or(crate::server::routing::user_variables::user_variables())
	.or(crate::server::routing::users::users())
	.or(crate::server::routing::plans::plans())
	.or(crate::server::routing::settings::settings())
	.or(crate::server::routing::events_scripts::events_scripts())
}

pub(crate) fn redirect_to_https(domorust: &Domorust) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::any()
		.and(warp::filters::host::optional())
		.and(warp::filters::path::full())
		.and(with_domorust(domorust.clone()))
		.and_then(crate::server::handlers::redirect_to_https)
}