use std::{collections::HashMap, error::Error, fmt::Debug};
use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable};
use serde::Serialize;

use crate::connection::IConnection;
use crate::plugins::py_domoticz::PyHardwareType;
use crate::{plugins, FromHashMap, FromSqlRow, FromSqlTable};
use crate::device::{Device, IDevice};
//use crate::plugins;
use crate:: utils::is_neg_or_zero;

#[allow(unused, non_snake_case)]
pub trait IHardware : Debug {
	fn on_start(&mut self) -> Result<(), Box<dyn Error>>;
	fn on_stop(&mut self) -> Result<(), Box<dyn Error>>;
	fn on_heartbeat(&mut self) -> Result<(), Box<dyn Error>>;
	
	fn on_device_added(&mut self, id:u32) -> Result<(), Box<dyn Error>>;
	fn on_device_modified(&mut self, id:u32) -> Result<(), Box<dyn Error>>;
	fn on_device_removed(&mut self, id:u32) -> Result<(), Box<dyn Error>>;
	fn on_security_event(&mut self, id:u32, Level:String, Description:String) -> Result<(), Box<dyn Error>>;

	fn on_connect(&mut self, connection:&dyn IConnection, Status: String, Description:String) -> Result<(), Box<dyn Error>>;
	fn on_disconnect(&mut self, connection:&dyn IConnection) -> Result<(), Box<dyn Error>>;
	fn on_message(&mut self, connection:&dyn IConnection, Data: &HashMap<String,String>) -> Result<(), Box<dyn Error>>;
	fn on_timeout(&mut self, connection:&dyn IConnection) -> Result<(), Box<dyn Error>>;
	#[allow(clippy::too_many_arguments)]
	fn on_notification(&mut self, name:String, Subject: String, Text: String, Status: String, Priority: String, Sound: String, ImageFile: String) -> Result<(), Box<dyn Error>>;

	fn on_command(&mut self, Unit: String, Command: String, Level: String, Color: String) -> Result<(), Box<dyn Error>>;

	fn has_manual_switches_support(&self) -> Result<bool, Box<dyn Error>>;
	fn get_manual_switches_json_configuration(&self) -> Result<String, Box<dyn Error>>;
	fn add_manual_switch(&mut self, name: String, SwitchType: u32, Type: u32, Parameters: &HashMap<String,String>) -> Result<(), Box<dyn Error>>;
	fn test_manual_switch(&mut self, SwitchType: u32, Type: u32, Parameters:&HashMap<String,String>) -> Result<(), Box<dyn Error>>;

	fn get_devices(&self) -> Vec<Device>;

	//fn clone_dyn(&self) -> Box<dyn IHardware+Send+Sync>;
}

pub trait IHardwareType : Debug {
	fn clone_dyn(&self) -> Box<dyn IHardwareType+Send+Sync>;
	fn get_data(&self) -> &HardwareTypeData;
}
#[derive(Debug, Clone)]
pub enum HardwareType {
	None,
	Python(PyHardwareType)
}
impl HardwareType {
	pub fn unwrap_python(self) -> PyHardwareType {
		match self {
			HardwareType::Python(h) => h,
			_ => panic!("unwrap_python on {:?}", self)
		}
	}
	pub fn is_python(&self) -> bool {
		match self {
			HardwareType::Python(_) => true,
			_ => false
		}
	}
	pub fn as_python_ref(&self) -> Option<&PyHardwareType> {
		match self {
			HardwareType::Python(ht) => Some(&ht),
			_ => None
		}
	}
	pub fn as_python_mut(&mut self) -> Option<&mut PyHardwareType> {
		match self {
			HardwareType::Python(ht) => Some(ht),
			_ => None
		}
	}
}
#[derive(Clone, Debug, Default, Serialize)]
#[derive(FromSqlRow, FromSqlTable, FromHashMap)]
#[table_name("Hardware")]
#[allow(non_snake_case)]
pub struct HardwareData {
	#[serde(rename="idx", with="crate::utils::string")]
	pub ID : usize,
	pub Name : String,
	pub Enabled : u8,
	pub Type : u32,
	pub Address : String,
	pub Port : u32,
	pub SerialPort : String,
	pub Username : String,
	pub Password : String,
	pub Extra : String,
	pub Mode1 : String,
	pub Mode2 : String,
	pub Mode3 : String,
	pub Mode4 : String,
	pub Mode5 : String,
	pub Mode6 : String,
	pub DataTimeout : u32,
	pub Configuration : String,
	pub LogLevel : u8,
}

#[derive(Clone, Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct HardwareTypeData
{
	pub ID:usize,
	pub key:String,
	pub name:String,
	pub wikiURL:String,
	pub externalURL:String,
	pub description:String,
	pub parameters:Vec<HardwareParameter>
}
impl HardwareTypeData {
	pub fn new(idx:usize, key:String, name:String) -> Self {
		HardwareTypeData{
			ID: idx,
			key,
			name,
			wikiURL:String::new(),
			externalURL:String::new(),
			description:String::new(),
			parameters:vec![]
		}
	}
}

#[derive(Clone, Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct HardwareParameter
{
	pub label:String,
	pub field:String,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub description:String,
	#[serde(skip_serializing_if = "Clone::clone")]
	pub required:bool,
	pub width:String,
	#[serde(skip_serializing_if = "is_neg_or_zero")]
	pub rows:i16,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub default:String,
	#[serde(skip_serializing_if = "Clone::clone")]
	pub password:bool,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub options:Vec<HardwareParameterOption>
}

impl HardwareParameter {
	pub fn new(label:String, field:String) -> Self {
		HardwareParameter{
			label,
			field,
			description:String::new(),
			required:false,
			width:String::new(),
			rows:-1,
			default:String::new(),
			password:false,
			options:vec![]
		}
	}
}
#[derive(Clone, Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct HardwareParameterOption
{
	pub label:String,
	pub value:String,
	#[serde(skip_serializing_if = "Clone::clone")]
	pub default:bool,
}
impl HardwareParameterOption {
	pub fn new(label:String, value:String, default:bool) -> Self {
		HardwareParameterOption{
			label,
			value,
			default
		}
	}
}
pub fn get_hardware_types_data() -> Result<Vec<HardwareTypeData>, Box<dyn Error>> {
	let types=plugins::get_plugins_hardware_types()?;
	let mut end_types=vec![];
	for t in &types {
		end_types.push(t.data.clone());
	}
	Ok(end_types)
}

