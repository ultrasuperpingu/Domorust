use std::{collections::HashMap, error::Error, path::PathBuf};

use pyo3::prelude::*;
use serde::Serialize;

use crate::{connection::IConnection, device::{Device, DeviceType}, hardware::{HardwareData, HardwareTypeData, IHardware, IHardwareType}};

use super::{module_call_function, module_call_function1, set_module_context, PyConnection};

#[pyfunction(name="Status")]
fn status(message: String) -> PyResult<()> {
	println!("{}", message);
	Ok(())
}
#[pyfunction(name="Error")]
fn error(message: String) -> PyResult<()> {
	println!("{}", message);
	Ok(())
}
#[pyfunction(name="Log")]
fn log(message: String) -> PyResult<()> {
	println!("{}", message);
	Ok(())
}
#[pyfunction(name="Connection")]
#[allow(unused)]
#[allow(non_snake_case)]
fn connection(Name: String, Transport: String, Protocol: String, Port: String) -> PyResult<PyConnection> {
	println!("{}", Name);
	Ok(PyConnection::default())
}
#[derive(Debug)]
pub struct PyHardware<'a> {
	pub data:HardwareData,
	pub module:Option<&'a Py<PyModule>>,
	pub devices:Vec<PyDevice>
}

#[allow(unused)]
impl<'a> IHardware for PyHardware<'a> {
	fn on_start(&mut self) -> Result<(), Box<dyn Error>> {
		let module = self.module.as_ref().unwrap();
		set_module_context(module, self);
		module_call_function(module, "onStart")?;
		Ok(())
	}

	fn on_stop(&mut self) -> Result<(), Box<dyn Error>> {
		let module = self.module.as_ref().unwrap();
		set_module_context(module, self);
		module_call_function(module, "onStop")?;
		Ok(())
	}

	fn on_heartbeat(&mut self) -> Result<(), Box<dyn Error>> {
		let module = self.module.as_ref().unwrap();
		set_module_context(module, self);
		module_call_function(module, "onHeartbeat")?;
		Ok(())
	}

	fn on_device_added(&mut self, id:u32) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_device_modified(&mut self, id:u32) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_device_removed(&mut self, id:u32) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_security_event(&mut self, id:u32, level:String, description:String) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_connect(&mut self, connection:&dyn IConnection, status: String, description:String) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_disconnect(&mut self, connection:&dyn IConnection) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_message(&mut self, connection:&dyn IConnection, data: &HashMap<String,String>) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_timeout(&mut self, connection:&dyn IConnection) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_notification(&mut self, name:String, subject: String, text: String, status: String, priority: String, sound: String, image_file: String) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn on_command(&mut self, unit: String, command: String, level: String, color: String) -> Result<(), Box<dyn Error>> {
		let module = self.module.as_ref().unwrap();
		set_module_context(module, self);
		module_call_function1(module, "onCommand", (unit, command, level, color))?;
		Ok(())
	}

	fn has_manual_switches_support(&self) -> Result<bool, Box<dyn Error>> {
		let module = self.module.as_ref().unwrap();
		set_module_context(module, self);
		let _=module_call_function(module, "hasManualSwitchesSupport")?;
		Ok(false)
	}

	fn get_manual_switches_json_configuration(&self) -> Result<String, Box<dyn Error>> {
		todo!()
	}

	fn add_manual_switch(&mut self, name: String, switch_type: u32, typ: u32, parameters: &HashMap<String,String>) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn test_manual_switch(&mut self, switch_type: u32, typ: u32, parameters:&HashMap<String,String>) -> Result<(), Box<dyn Error>> {
		todo!()
	}

	fn get_devices(&self) -> Vec<DeviceType> {
		let mut res=vec![];
		for d in &self.devices {
			res.push(d.clone().into())
		}
		res
	}

	/*fn clone_dyn(&self) -> Box<dyn IHardware+Send+Sync> {
		Box::new(self.clone())
	}*/

}
#[derive(Debug)]
#[allow(unused)]
pub struct PyHardwareType {
	pub data: HardwareTypeData,
	pub path: PathBuf,
	pub module:Option<Py<PyModule>>
}
impl PyHardwareType {
	pub fn new(idx:usize, key:String, name:String, path:PathBuf) -> Self {
		PyHardwareType{
			data:HardwareTypeData {
				ID: idx,
				key,
				name,
				wikiURL:String::new(),
				externalURL:String::new(),
				description:String::new(),
				parameters:vec![]
			},
			path:path.clone(),
			module:None
		}
	}
}
impl Clone for PyHardwareType {
	fn clone(&self) -> Self {
		Self { data: self.data.clone(), path: self.path.clone(), module: None }
	}
}
impl IHardwareType for PyHardwareType {
	fn clone_dyn(&self) -> Box<dyn IHardwareType+Send+Sync> {
		Box::new(self.clone())
	}
	fn get_data(&self) -> &HardwareTypeData {
		&self.data
	}
}

#[derive(Clone, Debug, Default, Serialize)]
#[pyclass]
#[pyo3(name="Device")]
pub struct PyDevice {
	pub device:Device
}
impl Into<DeviceType> for PyDevice {
	fn into(self) -> DeviceType {
		DeviceType::Python(self)
	}
}
#[pymethods]
#[allow(non_snake_case)] #[allow(unused_variables)]
impl PyDevice {
	#[new]
	#[pyo3(signature = (Name,Unit,TypeName="",Type=1,Subtype=1,Switchtype=1,Image=-1,Used=false,DeviceID=""))]
	#[allow(clippy::too_many_arguments)]
	pub fn new(Name:&str, Unit:usize, TypeName:&str, Type:usize, Subtype:usize, Switchtype:usize, Image:i32, Used:bool, DeviceID:&str) -> Self {
		let d=Device {
			Name: String::from(Name),
			Unit,
			Type,
			TypeName: String::from(TypeName),
			SubType: Subtype,
			Image: Image.to_string(),
			Used: if Used {1} else {0},
			ID: DeviceID.parse().unwrap_or_default(),
			..Default::default()
	 	};
		//if d.Type.is_empty() {
		//	d.Type=Type.to_string();
		//}
		PyDevice{device:d}
	}
	#[getter]
	#[pyo3(name="Name")]
	pub fn get_name(&self) -> PyResult<String> {
		Ok(self.device.Name.clone())
	}
	#[getter]
	#[pyo3(name="DeviceID")]
	pub fn get_device_id(&self) -> PyResult<String> {
		Ok(self.device.DeviceID.clone())
	}
	pub fn Create(&mut self)
	{
		println!("Create Device");
	}
	pub fn Update(&mut self, _val: &str)
	{
		println!("Update Device");
	}
	pub fn Delete(&mut self)
	{
		println!("Delete Device");
	}
}
pub fn get_pydevice(d:Device) -> PyDevice {
	PyDevice{device:d}
}
pub fn get_pydevices(datas: Vec<Device>) -> Vec<PyDevice> {
	let mut res=vec![];
	for d in datas {
		res.push(get_pydevice(d));
	}
	res
}
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule()] #[pyo3(name="Domoticz")]
pub fn domoticz(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(status, m)?)?;
	m.add_function(wrap_pyfunction!(error, m)?)?;
	m.add_function(wrap_pyfunction!(log, m)?)?;
	//m.add_function(wrap_pyfunction!(connection, m)?)?;
	m.add_class::<PyDevice>()?;
	m.add_class::<PyConnection>()?;
	Ok(())
}
