
use crate::hardware::HardwareType;
use crate::plugins::{get_plugins_hardware_types, load_module, prepare_python_context};

#[derive(Clone, Debug)]
pub struct DomorustConfig {
	/// Server port for http
	pub port: u16,
	/// Path of the web resources
	pub wwwroot: std::path::PathBuf,
	// Server port for https
	pub sslport : u16,
	// Server https certificate
	pub cert: Option<std::path::PathBuf>,
	// Server https private key
	pub private_key: Option<std::path::PathBuf>,
	// Don't redirect from http to https
	pub no_redirect_to_https: bool,
	
}
#[derive(Debug, Clone)]
pub struct Domorust {
	pub cli: DomorustConfig,
	pub hardware_types: Vec<HardwareType>,
	//pub hardwares : Vec<Box<dyn IHardware+Send+Sync>>
}
// will probably need this: check if it is safe...
//unsafe impl Send for Domorust {}
/*impl Clone for Box<dyn IHardwareType+Send+Sync> {
	fn clone(&self) -> Self {
		self.clone_dyn()
	}
}
impl Clone for Box<dyn IHardware+Send+Sync> {
	fn clone(&self) -> Self {
		self.clone_dyn()
	}
}*/
impl Domorust {
	pub fn new(cli:DomorustConfig) -> Self {
		let hts=get_plugins_hardware_types().expect("Can't get plugin hardware");
		//let hws=get_plugin_hardwares().expect("Can't get plugin hardware");
		let mut instance=Domorust{hardware_types:vec![], /*hardwares:vec![],*/ cli};
		prepare_python_context();
		for mut h in hts {
			load_module(&mut h).unwrap();
			instance.hardware_types.push(HardwareType::Python(h));
		}

		/*for h in hws {
			instance.hardwares.push(Box::new(h));
		}*/
		instance
	}
}