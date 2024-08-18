mod db;
mod domoticz;
mod handlers;

mod server;

use std::{collections::HashMap, error::Error};

use clap::Parser;
use db::hardwares::get_hardwares_data;
use serde::Serialize;
use server::routing;
use warp::Filter;
use chrono::Local;
use chrono::DateTime;

use domorust_models::timers::get_next_time_of_timer;
use domorust_models::timers::Timer;
use domorust_models::domorust::{Domorust, DomorustConfig};
use domorust_models::plugins::py_domoticz::PyHardware;
use domorust_models::plugins::{set_module_context, module_call_function};
use domorust_models::plugins::py_domoticz::get_pydevices;


//TODO: just a dirty hack. need to be implemeted properly
pub fn get_plugin_hardwares() -> Result<Vec<PyHardware<'static>>, Box<dyn Error>> {
	let mut hws=vec![];
	let hardware_datas=get_hardwares_data(HashMap::new())?;
	for h in hardware_datas {
		let devices = get_pydevices(db::devices::get_hardware_devices(h.ID, HashMap::new()).unwrap());
		hws.push(PyHardware{data:h, module:None, devices});
	}
	Ok(hws)
}
//TODO: just a dirty hack. need to be implemeted properly
pub fn setup_timers() {
	let timers=crate::db::timers::get_timers(HashMap::new()).unwrap();
	let gps=db::get_latitude_longitude().unwrap_or_default();
	let (latitude, longitude) = (gps.Latitude, gps.Longitude);
	let now = Local::now();
	for t in &timers {
		setup_timer(t, now, latitude, longitude);
	}
}
fn setup_timer(t: &Timer, now: DateTime<Local>,latitude:f64, longitude:f64) {
	let cmd=t.Cmd;
	let dev_id=t.DeviceRowID;
	let t=t.clone();
	match get_next_time_of_timer(&t, latitude, longitude) {
		Ok(d) => {
			tokio::spawn(async move {
				let duration = d.signed_duration_since(now).to_std();
				if let Ok(duration) = duration {
					tokio::time::sleep(duration).await;
					println!("command {}", cmd);
					let _device = db::devices::get_device(dev_id, HashMap::new()).unwrap();
					//TODO: send command
					setup_timer(&t, now, latitude, longitude);
				} else {
					eprintln!("Error preparing timer: d:{:?}, duration:{:?}", d, duration);
				}
			});
		},
		Err(f) => eprintln!("Error preparing timer: {:?}", f),
	}
}
//TODO: just a dirty hack. need to be implemeted properly
pub fn call_onstart_on_plugins() -> Result<(), Box<dyn Error>>{
	let types=&DOMORUST.get().unwrap().hardware_types;
	let hws=get_plugin_hardwares()?;
	//let hws = DOMORUST.get()?;
	for hw in hws {
		let typ=types.iter().find(|t| {hw.data.Extra == t.as_python_ref().unwrap().data.key});
		if let Some(t) = typ {
			let t=t.as_python_ref().unwrap();
			//load_module(t)?;
			set_module_context(t.module.as_ref().unwrap(), &hw)?;
			module_call_function(t.module.as_ref().unwrap(), "onStart")?;
		}
	}
	Ok(())
}

#[derive(Parser, Debug, Serialize, Clone)]
#[command(version, about, long_about = None)]
/// DomoRust: Domotic solution implemented with rust
struct Cli {
	/// Server port for http
	#[arg(short, long, default_value_t=8081)]
	port: u16,
	/// Path of the web resources
	#[arg(long, default_value="www")]
	wwwroot: std::path::PathBuf,
	// Server port for https
	#[arg(short, long, default_value_t=2443)]
	sslport : u16,
	// Server https certificate file
	#[arg(long)]
	cert: Option<std::path::PathBuf>,
	// Server https private key file
	#[arg(long)]
	private_key: Option<std::path::PathBuf>,
	// Don't redirect from http to https
	#[arg(short, long, action, default_value="false")]
	no_redirect_to_https: bool,
}
impl Cli {
	fn as_domorust_config_ref(&self) -> DomorustConfig {
		DomorustConfig {
			port: self.port,
			wwwroot: self.wwwroot.clone(),
			sslport : self.sslport,
			cert: self.cert.clone(),
			private_key: self.private_key.clone(),
			no_redirect_to_https: self.no_redirect_to_https,
		}
	}
}
pub static DOMORUST: tokio::sync::OnceCell<Domorust> = tokio::sync::OnceCell::const_new();
#[tokio::main]
async fn main() {
	let args = Cli::parse();
	let port = args.port;
	let sslport = args.sslport;
	let cert = args.cert.clone();
	let private_key = args.private_key.clone();
	let use_https = cert.is_some() && private_key.is_some();
	let redirect_http = use_https && !args.no_redirect_to_https;
	DOMORUST.set(Domorust::new(args.as_domorust_config_ref())).unwrap();
	db::init_db();
	let res=call_onstart_on_plugins();
	println!("plugin read: {:?}",res);
	let domorust=DOMORUST.get().unwrap();
	let routes = routing::routes(domorust);
	setup_timers();
	//setup_hardwares_lifetime(domorust);
	if !use_https {
		warp::serve(routes.clone())
		.run(([0, 0, 0, 0], port))
		.await;
	} else if redirect_http {
		let (_http_addr, http_warp)=warp::serve(routing::redirect_to_https(domorust))
			//.run(([0, 0, 0, 0], port))
			.bind_ephemeral(([0, 0, 0, 0], port));
			//.await;

		let (_https_addr, https_warp) = warp::serve(routes)
			.tls()
			.cert_path(cert.unwrap())
			.key_path(private_key.unwrap())
			//.run(([0, 0, 0, 0], sslport))
			.bind_ephemeral(([0, 0, 0, 0], sslport));
			//.await;
		//futures::future::join(http_warp, https_warp).await;
		tokio::join!(http_warp, https_warp);
	} else {
		let (_http_addr, http_warp)=warp::serve(routes.clone())
			//.run(([0, 0, 0, 0], port))
			.bind_ephemeral(([0, 0, 0, 0], port));
			//.await;

		let (_https_addr, https_warp) = warp::serve(
				routing::redirect_to_https(domorust).or(routes)
			)
			.tls()
			.cert_path(cert.unwrap())
			.key_path(private_key.unwrap())
			//.run(([0, 0, 0, 0], sslport))
			.bind_ephemeral(([0, 0, 0, 0], sslport));
			//.await;
		//futures::future::join(http_warp, https_warp).await;
		tokio::join!(http_warp, https_warp);
	}
}
