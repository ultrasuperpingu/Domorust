use std::collections::HashMap;
use std::convert::Infallible;

use crate::server::responses::{ConfigResponse, RequestError, RequestResult};

use bytes::Buf;
use futures_util::TryStreamExt;
use warp::reply::Reply;

pub async fn json_htm_get(path:warp::filters::path::FullPath, params: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
	println!("GET params:{:?} on {:?}", params, path);
	let empty=String::from("");
	let t = params.get("type").unwrap_or(&empty);
	let param = params.get("param").unwrap_or(&empty);
	let out_res=RequestError::new("Unknown", "Unknown resource".into());
	if t == "command" {
		if param == "getconfig" {
			return Ok(warp::reply::with_status(warp::reply::json(&ConfigResponse::default()), warp::http::StatusCode::OK))
			//return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "getdynamicpricedevices" {
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "gettransfers" {
			let idx_str = params.get("idx").unwrap_or(&empty);
			if !idx_str.is_empty() {
				if let Ok(_idx) = idx_str.parse::<u16>() {
					//TODO
				}
			}
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "switchdeviceorder" {
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "switchmodal" {
			//type=command&param=deletecustomicon"&idx=0&status=&action=1
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "setcolbrightnessvalue" {
			//type=command&param=setcolbrightnessvalue&idx=0&color=&brightness=12
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "getcameras" {
			//type=command&param=getcameras&order=Name&displayhidden=1
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "getfloorplanplans" {
			//type=command&param=getfloorplanplans&idx=0
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "getfloorplans" {
			//type=command&param=getfloorplans
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "testswitch" {
			//type=command&param=testswitch (+params)
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "addswitch" {
			//type=command&param=addswitch (+params)
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "getmanualhardware" {
			//type=command&param=getmanualhardware
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "learnsw" {
			//type=command&param=learnsw
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "setused" {
			//type=command&param=setused&idx=" + $.devIdx + '&name=' + encodeURIComponent($("#dialog-addlightdevice #devicename").val()) + '&switchtype=' + $("#dialog-addlightdevice #comboswitchtype").val() + '&used=true&maindeviceidx=' + MainDeviceIdx
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "gettitle" {
			return Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
		} else if param == "getsubdevices" {
			return Ok(warp::reply::with_status(warp::reply::json(&RequestResult::<String>::new("GetSubDevices", vec![])), warp::http::StatusCode::OK))
		}
		
	}
	//Ok(warp::reply::with_status(warp::reply::json(&out_res), warp::http::StatusCode::NOT_IMPLEMENTED))
	//Ok(warp::reply::json(&out_res))
	Ok(warp::reply::with_status(warp::reply::json(&RequestError::new("Unknown", "Unknown".into())), warp::http::StatusCode::NOT_FOUND))
}

pub async fn json_htm_post(params: HashMap<String, String>, content: warp::multipart::FormData) -> Result<impl warp::Reply, Infallible> {
	let test:Vec<(String,String,String)>=content.and_then(|mut field| async move {
		let mut bytes: Vec<u8> = Vec::new();
		// field.data() only returns a piece of the content, you should call over it until it replies None
		while let Some(content) = field.data().await {
			let mut content = content.unwrap();
			while content.has_remaining() {
				bytes.push(content.get_u8());
			}
		}
		Ok((
			field.name().to_string(),
			field.filename().unwrap_or("").to_string(),
			String::from_utf8_lossy(&bytes).to_string(),
		))
	}).try_collect().await.unwrap();
	println!("POST: params:{:?}\n\tcontent:{:?}", params, test);
	Ok(warp::reply::with_status(warp::reply::json(&RequestError::new("Unknown", "Unknown".into())), warp::http::StatusCode::NOT_FOUND).into_response())
}