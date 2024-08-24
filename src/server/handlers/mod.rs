use std::str::FromStr;

use warp::{filters::host::Authority, reject::Rejection, reply::{self, Reply}};

use crate::DOMORUST;

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

pub async fn redirect_to_https(authority: Option<Authority>, path:warp::filters::path::FullPath) -> Result<impl Reply, Rejection> {
	let domorust = DOMORUST.get().unwrap();
	let authority = authority.unwrap();
	println!("{}{}", authority, path.as_str());
	if authority.port_u16() != Some(domorust.cli.port) {
		return Err(warp::reject::reject());
	}
	
	let url="https://".to_string()+authority.host()+":"+domorust.cli.sslport.to_string().as_str();
	println!("redirect to {}", url);
	if let Ok(uri) = warp::http::Uri::from_str(url.as_str()) {
		Ok(warp::redirect(uri).into_response())
	} else {
		Ok(reply::with_status(reply::json(&"res"), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
	}
}