use std::{collections::HashMap, convert::Infallible};

use bytes::BufMut;
use futures_util::TryStreamExt;
use warp::filters::multipart::{FormData, Part};
use warp::reject::Rejection;
use warp::reply::{self, Reply};
use warp::http::StatusCode;

use crate::db;
use crate::server::responses::{RequestError, RequestResult};


pub async fn get_custom_icons(params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::custom_icons::get_custom_icons(params) {
		Ok(res) => {
			Ok(reply::json(&RequestResult::new("GetCustomIcons", res)).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("GetCustomIcons",e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_custom_icon_small_image(idx:usize) -> Result<impl warp::Reply, Infallible> {
	match db::custom_icons::get_custom_icons_small_image(idx) {
		Ok(data) => {
			Ok(reply::with_header(data, "Content-Type", "image/png").into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_custom_icon_on_image(idx:usize) -> Result<impl warp::Reply, Infallible> {
	match db::custom_icons::get_custom_icons_on_image(idx) {
		Ok(data) => {
			Ok(reply::with_header(data, "Content-Type", "image/png").into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn get_custom_icon_off_image(idx:usize) -> Result<impl warp::Reply, Infallible> {
	match db::custom_icons::get_custom_icons_off_image(idx) {
		Ok(data) => {
			Ok(reply::with_header(data, "Content-Type", "image/png").into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("Plans", e)), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}
pub async fn add_custom_icon(form: FormData,params: HashMap<String, String>) -> Result<impl warp::Reply, Rejection> {
	let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
		eprintln!("form error: {}", e);
		warp::reject::reject()
	})?;

	for p in parts {
		if p.name() == "file" {
			let content_type = p.content_type();
			let file_ending;
			match content_type {
				Some(file_type) => match file_type {
					"application/pdf" => {
						file_ending = "pdf";
					}
					"image/png" => {
						file_ending = "png";
					},
					"image/bmp" => {
						file_ending = "bmp";
					},
					"image/jpeg" => {
						file_ending = "jpg";
					}
					v => {
						eprintln!("invalid file type found: {}", v);
						return Err(warp::reject::reject());
					}
				},
				None => {
					eprintln!("file type could not be determined");
					//return Err(warp::reject::reject());
					file_ending = "png";
				}
			}

			let value = p
				.stream()
				.try_fold(Vec::new(), |mut vec, data| {
					vec.put(data);
					async move { Ok(vec) }
				})
				.await
				.map_err(|e| {
					eprintln!("reading file error: {}", e);
					warp::reject::reject()
				})?;

			let file_name = format!("./files/{}.{}", "ploup", file_ending);
			tokio::fs::write(&file_name, value).await.map_err(|e| {
				eprint!("error writing file: {}", e);
				warp::reject::reject()
			})?;
			println!("created file: {}", file_name);
		}
	}

	match db::custom_icons::add_custom_icon(params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("AddCustomIcon", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("AddCustomIcon", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn update_custom_icon(idx: usize, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::custom_icons::update_custom_icon(idx, params) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("UpdateCustomIcon", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("UpdateCustomIcon", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn delete_custom_icon(idx: usize, _params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	match db::custom_icons::delete_custom_icon(idx) {
		Ok(()) => {
			Ok(warp::reply::json(&RequestResult::<String>::new("DeleteCustomIcon", vec![])).into_response())
		},
		Err(e) => {
			Ok(reply::with_status(reply::json(&RequestError::new("DeleteCustomIcon", e)), StatusCode::INTERNAL_SERVER_ERROR).into_response())
		}
	}
}

pub async fn get_images() -> Result<impl warp::Reply, Infallible> {
	Ok(reply::json(&RequestResult::new("GetImages", domorust_models::custom_images::get_images())).into_response())
}