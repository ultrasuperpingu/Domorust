
use warp::Filter;


pub(crate) fn infos() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	get_version().or(get_uptime()).or(get_sun_rise_set())
	.or(get_serial_devices()).or(get_hardwaretypes())
	.or(get_switchtypes()).or(get_metertypes()).or(get_languages())
	.or(get_config())
}

fn get_version() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "version")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_version)
}
fn get_uptime() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "uptime")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_uptime)
}
fn get_sun_rise_set() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "sun_rise_set")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_sun_rise_set)
}
fn get_serial_devices() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "serial_devices")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_serial_devices)
}
fn get_hardwaretypes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "hardwaretypes")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_hardwaretypes)
}
fn get_switchtypes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "switchtypes")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_switchtypes)
}
fn get_metertypes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "metertypes")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_metertypes)
}
fn get_languages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "languages")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_languages)
}
fn get_config() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::path!("domorust-api" / "infos" / "config")
		.and(warp::get())
		.and_then(crate::server::handlers::infos::get_config)
}
