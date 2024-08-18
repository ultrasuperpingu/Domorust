use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use py_domoticz::{domoticz, PyDevice, PyHardware, PyHardwareType};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};
use serde::Serialize;

use crate::connection::IConnection;
//use crate::db::hardwares::get_hardwares_data;
use crate::device::DeviceData;
use crate::hardware::{HardwareParameter, HardwareParameterOption, HardwareTypeData, IHardware};


pub mod py_domoticz;

#[derive(Clone, Debug, Default, Serialize)]
#[pyclass(name="Connection")]
#[allow(non_snake_case)]
pub struct PyConnection
{
	pub Name: String,
	//pub Target,
	pub Address: String,
	pub Port: u32,
	pub Baud: u32,
	pub Timeout: i32,
	//pub LastSeen,
	//pub pPlugin: &Hardware,
	//pub Parent: &Connection
}
#[pymethods] #[allow(non_snake_case)]
impl PyConnection {
	#[allow(unused)]
	#[new]
	#[pyo3(signature = (Name,Transport,Protocol,Address="",Port="",Baud=0))]
	pub fn new(Name:&str, Transport:&str, Protocol:&str, Address: &str, Port:&str, Baud:u32) -> PyConnection
	{
		PyConnection {
			Name: String::from(Name),
			Address: String::from(Address),
			Port: Port.parse().unwrap_or(0),
			Baud,
			..Default::default()
		}
	}

	#[allow(unused)]
	#[pyo3(name="Connect", signature = (Target="",Timeout=0))]
	fn py_connect(&mut self, Target:&str, Timeout:u32) { self.connect(Timeout) }
	
	#[allow(unused)]
	#[pyo3(name="Send", signature = (Message, Delay=0.0))]
	//TODO: PyAny for message
	fn py_send(&mut self, Message:String/*, Byte, ByteArray, Object*/, Delay: f32) { self.send(Message, Delay) }
	
	#[allow(unused)]
	#[pyo3(name="Listen", signature = (Target=""))]
	fn py_listen(&mut self, Target: &str) { self.listen() }
	
	#[pyo3(name="Disconnect")]
	fn py_disconnect(&mut self) { self.disconnect() }
	
	#[pyo3(name="Connecting")]
	fn py_connecting(&mut self) -> bool { self.connecting() }

	#[pyo3(name="Connected")]
	fn py_connected(&mut self) -> bool { self.connected() }

	#[pyo3(name="LastSeen")]
	fn py_last_seen(&mut self) -> String { self.last_seen().to_string() }
}
impl IConnection for PyConnection {
	#[allow(unused)]
	fn connect(&mut self, timeout:u32){}
	
	#[allow(unused)]
	fn send(&mut self, message:String/*, Byte, ByteArray, Object*/, delay: f32){}
	
	#[allow(unused)]
	fn listen(&mut self){}
	
	fn disconnect(&mut self){}
	
	fn connecting(&self) -> bool {false}

	fn connected(&self) -> bool {false}

	fn last_seen(&self) -> DateTime<Utc> { Utc::now() }
}

#[allow(dead_code)]
fn test_launch_on_start(path: &PathBuf) -> PyResult<()>
{
	Python::with_gil(|py| {
		let sys = py.import_bound("sys")?;
		let syspath = sys.getattr("path")?.downcast_into::<PyList>()?;
		syspath.insert(0, path)?;
		//let locals = [("os", py.import_bound("os")?),("Domoticz", py.import_bound("Domoticz")?)].into_py_dict_bound(py);
		let plugin = PyModule::import_bound(py, "plugin")?;
		let _ = plugin.add("Parameters", [("Mode1", ""),("Mode2",r###"{
			"devices":[
			{"name":"Volet Salon2","localip":"fe80::dcf6:e5ff:fec9:4276","type":"4098","pid":"13920","addr":"14429805"},
			{"name":"Volet Cuisine2","localip":"fe80::dcf6:e5ff:fec9:4276","type":"4098","pid":"13920","addr":"5897550"},
			{"name":"Ecran Up","localip":"fe80::dcf6:e5ff:fec9:4276","type":"4096","opt":"17","pid":"801","addr":"557494"},
			{"name":"Ecran Down","localip":"fe80::dcf6:e5ff:fec9:4276","type":"4096","opt":"20","pid":"801","addr":"557494"}
			]
			}"###),("Mode3",""),("Mode4",""),("Mode5",""),("Mode6",""),("Address",""),("Port","")].into_py_dict_bound(py));
		let devices=HashMap::<u32, Bound<DeviceData>>::new();
		let _ = plugin.add("Devices", devices.into_py_dict_bound(py));
		let _test = plugin.getattr("onStart")?.call0();
		//println!("{:?}",test);
		_=syspath.del_item(0);
		Ok(())
	})
}

pub fn load_module(hardware_type: &mut PyHardwareType) -> PyResult<()>
{
	//println!("{:?}",hardware_type);
	Python::with_gil(|py| {
		let sys = py.import_bound("sys")?;
		let syspath = sys.getattr("path")?.downcast_into::<PyList>()?;
		syspath.insert(0, hardware_type.path.parent())?;
		
		let code = std::fs::read_to_string(&hardware_type.path)?;
		let plugin = PyModule::from_code_bound(py, &code, "plugin.py",hardware_type.data.key.as_str())?;
		hardware_type.module = Some(plugin.unbind());
		Ok(())
	})
}
pub fn set_module_context(module: &Py<PyModule>, hardware: &PyHardware) -> PyResult<()>
{
	Python::with_gil(|py| {
		let plugin = module.bind(py);
		let _ = plugin.add("Parameters", [
			("Mode1", hardware.data.Mode1.clone()),
			("Mode2",hardware.data.Mode2.clone()),
			("Mode3",hardware.data.Mode3.clone()),
			("Mode4",hardware.data.Mode4.clone()),
			("Mode5",hardware.data.Mode5.clone()),
			("Mode6",hardware.data.Mode6.clone()),
			("Address",hardware.data.Address.clone()),
			("Port",hardware.data.Port.clone().to_string())].into_py_dict_bound(py));
		let mut devices = HashMap::<String, Bound<PyDevice>>::new();
		let devices_vec = hardware.get_devices();
		for dev in devices_vec {
			let d=dev.as_python_ref().unwrap();
			devices.insert(d.device.Unit.to_string(), Bound::new(py, d.clone()).unwrap());
		}
		let _ = plugin.add("Devices", devices.into_py_dict_bound(py));
		Ok(())
	})
}
pub fn module_call_function(module: &Py<PyModule>, func_name: &str) -> PyResult<()>
//pub fn module_call_function<T : for<'a> From<pyo3::Bound<'a, pyo3::PyAny>>>(module: &Py<PyModule>, func_name: &str) -> PyResult<T>
{
	Python::with_gil(|py| {
		let plugin = module.bind(py);
		let _ = plugin.getattr(func_name)?.call0()?;

		Ok(())
	})
}
pub fn module_call_function1<T : pyo3::IntoPy<pyo3::Py<pyo3::types::PyTuple>>>(module: &Py<PyModule>, func_name: &str, param: T) -> PyResult<()>
{
	Python::with_gil(|py| {
		let plugin = module.bind(py);
		let _ = plugin.getattr(func_name)?.call1(param)?;
		Ok(())
	})
}
pub fn get_plugins_hardware_types() -> Result<Vec<PyHardwareType>, Box<dyn Error>>
{
	let mut htypes=vec![];
	let paths = fs::read_dir("plugins")?;
	let mut idx = 0;
	for path in paths {
		let path=path?.path();
		if path.is_dir() {
			let dirpath_str = path.file_name().unwrap().to_str().unwrap();
			let plugin_path = path.join("plugin.py");
			if plugin_path.exists() {
				println!("Found plugin: {}", plugin_path.display());
				let contents = fs::read_to_string(&plugin_path)?;
				let desc_idx=contents.find("<plugin");
				if desc_idx.is_none() {
					continue;
				}
				let desc_end_idx=contents.find("</plugin>");
				if desc_end_idx.is_none() {
					continue;
				}
				//println!("Desc: {}", &contents[desc_idx.unwrap()..desc_end_idx.unwrap()+9]);
				let doc = roxmltree::Document::parse(&contents[desc_idx.unwrap()..desc_end_idx.unwrap()+9]).unwrap();
				if doc.root_element().has_attribute("key") {
					let key = doc.root_element().attribute("key").unwrap();
					if dirpath_str != key {
						eprintln!("Plugin key {} does not match dir name {}", key, dirpath_str);
						//continue;
					}
					let name =  if !doc.root_element().has_attribute("name") {
						key
					} else {
						doc.root_element().attribute("name").unwrap()
					};
					let mut ht=HardwareTypeData::new(idx, String::from(key),String::from(name));
					for e in doc.root_element().children() {
						if e.is_element() && e.has_tag_name("description") {
							ht.description=String::from(e.text().unwrap_or_default());
						}
						if e.is_element() && e.has_tag_name("params") {
							for p in e.children() {
								if p.is_element() && p.has_tag_name("param") {
									let label = p.attribute("label");
									let field = p.attribute("field");
									if field.is_some() && label.is_some() {
										let mut param=HardwareParameter::new(String::from(label.unwrap()),String::from(field.unwrap()));
										param.password = p.has_attribute("password") && p.attribute("password").unwrap().to_lowercase() == "true";
										param.default = if p.has_attribute("default") { String::from(p.attribute("default").unwrap()) } else {String::new()};
										param.width = if p.has_attribute("width") { String::from(p.attribute("width").unwrap()) } else {String::new()};
										param.rows = if p.has_attribute("rows") { String::from(p.attribute("rows").unwrap()).parse().unwrap_or(-1) } else {-1};
										for child in p.children() {
											if child.is_element() && child.has_tag_name("description") {
												param.description = String::from(child.text().unwrap_or_default());
											}
											else if child.is_element() && child.has_tag_name("options") {
												let mut options: Vec<HardwareParameterOption>=vec![];
												for opt in child.children() {
													if opt.is_element() && opt.has_tag_name("option") {
														options.push(HardwareParameterOption::new(
															String::from(opt.attribute("label").unwrap()),
															String::from(opt.attribute("value").unwrap()),
															if opt.has_attribute("default") {String::from(opt.attribute("default").unwrap()).to_lowercase() == "true"} else { false })
														);
													}
												}
												param.options = options;
											}
										}
										ht.parameters.push(param);
									}
								}
										
							}
						}
					}
					let pyht=PyHardwareType{data:ht,path:plugin_path.clone(), module:None};
					htypes.push(pyht);
					idx+=1;
				}
			}
		}
	}
	Ok(htypes)
}



pub fn prepare_python_context() {
	pyo3::append_to_inittab!(domoticz);
	pyo3::prepare_freethreaded_python();
}