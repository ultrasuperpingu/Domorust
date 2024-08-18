#![allow(unused)]

use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};

use domorust_macros::{FromHashMap, FromSqlRow, FromSqlRowFields, FromSqlTable};
use crate::{plugins::py_domoticz::PyDevice, utils::is_nan_f32, FromHashMap, FromSqlRow, FromSqlRowFields, FromSqlTable, ParseDomorustTypeError};

pub trait IDevice {
	fn on_device_changed(&mut self);
}

pub trait ISensor : IDevice {
	
}

pub trait IEffector : IDevice {
	fn send_command(&mut self, command:u32);
}
#[derive(Debug, Clone)]
pub enum Device {
	None,
	Python(PyDevice)
}
impl Device {
	pub fn unwrap_python(self) -> PyDevice {
		match self {
			Device::Python(h) => h,
			_ => panic!("unwrap_python on {:?}", self)
		}
	}
	pub fn is_python(&self) -> bool {
		match self {
			Device::Python(_) => true,
			_ => false
		}
	}
	pub fn as_python_ref(&self) -> Option<&PyDevice> {
		match self {
			Device::Python(ht) => Some(&ht),
			_ => None
		}
	}
	pub fn as_python_mut(&mut self) -> Option<&mut PyDevice> {
		match self {
			Device::Python(ht) => Some(ht),
			_ => None
		}
	}
}
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Serialize)]
#[derive(FromHashMap, FromSqlRow, FromSqlTable)]
#[table_name("DeviceStatus")]
pub struct DeviceData {
	pub AddjMulti : f32,
	pub AddjMulti2 : f32,
	pub AddjValue : f32,
	pub AddjValue2 : f32,
	pub BatteryLevel : u8,
	#[skip_field]
	#[serde(skip_serializing_if="crate::utils::is_u8_max")]
	pub Barometer : u8,
	pub Color : Option<String>,
	pub CustomImage : usize,
	#[column_name("sValue")]
	pub Data : String,
	pub Description : String,
	pub DeviceID : String,
	#[skip_field]
	pub DewPoint : f32,
	#[param_name("favorite")]
	pub Favorite : u8,
	#[skip_field]
	pub HardwareDisabled : bool,
	pub HardwareID : usize,
	#[skip_field]
	pub HardwareName : String,
	#[skip_field]
	pub HardwareType : String,
	#[skip_field]
	pub HardwareTypeVal : u32,
	#[skip_field]
	pub HaveTimeout : bool,
	#[skip_field]
	#[serde(skip_serializing_if="crate::utils::is_nan_f32")]
	pub Humidity : f32,
	#[skip_field]
	pub HumidityStatus : String,
	#[skip_field]
	pub Image : String,
	pub LastUpdate : String,
	//#[skip_field]
	//pub LevelNames : String,
	#[skip_field]
	pub LevelInt : u8,
	pub Name : String,
	#[skip_field]
	pub Notifications : bool,
	#[serde(flatten)]
	pub Options : Option<NameValueList>,
	#[skip_field]
	pub PlanID : String,
	#[skip_field]
	pub PlanIDs : Vec<String>,
	pub Protected : bool,
	#[skip_field]
	pub ShowNotifications : bool,
	#[skip_field]
	pub Status : String,
	pub SignalLevel : i32,
	#[serde(rename="SubTypeVal")]
	pub SubType : usize,
	#[skip_field]
	#[serde(rename="SubType")]
	pub SubTypeName : String,
	#[serde(rename="SwitchTypeVal")]
	pub SwitchType : usize,
	#[skip_field]
	#[serde(rename="SwitchType")]
	pub SwitchTypeName : String,
	#[skip_field]
	#[serde(rename="Temp", skip_serializing_if="crate::utils::is_nan_f32")]
	pub Temperature : f32,
	#[skip_field]
	pub Timers : bool,
	#[serde(rename="TypeVal")]
	pub Type : usize,
	#[skip_field]
	#[serde(rename="Type")]
	pub TypeName : String,
	#[skip_field]
	pub TypeImg : String,
	#[skip_field]
	pub UsedByCamera : bool,
	pub Unit : usize,
	#[param_name("used")]
	pub Used : u8,
	#[skip_field]
	pub XOffset : u16,
	#[skip_field]
	pub YOffset : u16,
	#[param_name("idx")]
	#[serde(rename="idx", with="crate::utils::string")]
	pub ID : usize,
	#[skip_field]
	pub trend : u16
}

#[derive(Debug, Clone, Serialize)]
pub struct NameValueList(
	HashMap<String,String>
);
impl FromSql for NameValueList {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		value.as_str()?.parse()
			.map_err(|e| FromSqlError::Other(Box::new(e)))
	}
}
impl ToSql for NameValueList {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok(self.to_string().into())
	}
}
impl FromStr for NameValueList {
	type Err = ParseDomorustTypeError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Ok(NameValueList(HashMap::new()))
		}
		let opts_str : Vec<&str> = s.split(';').collect();
			//.ok_or(ParseDomorustTypeError{})?;
		let mut opts = HashMap::new();

		for (index, opt) in opts_str.into_iter().enumerate() {
			let res = opt.split_once(':');//.ok_or(ParseDomorustTypeError::new("Options", s.to_string()))?;
			if let Some((name, val)) = res {
				opts.insert(String::from(name), String::from(val));
			} else {
				opts.insert(String::from("item")+index.to_string().as_str(), String::from(opt));
			}
		}
		Ok(NameValueList(opts))
	}
}

impl Display for NameValueList{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut res=String::new();
		for (name, val) in &self.0 {
			res+=name.as_str();
			res+=":";
			res+=val.as_str();
			res+=";";
		}
		let res=res.trim_end_matches(';');
		write!(f, "{}", res)
	}
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TempHumBaroData {
	pub d:String,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub hu:String,
	#[serde(skip_serializing_if = "is_nan_f32")]
	pub te:f32,
	#[serde(skip_serializing_if = "is_nan_f32")]
	pub tm: f32,
	#[serde(skip_serializing_if = "is_nan_f32")]
	pub ta: f32,
	#[serde(skip_serializing_if = "String::is_empty")]
	pub ba: String,
	#[serde(skip_serializing_if = "is_nan_f32")]
	pub ch:f32,
	#[serde(skip_serializing_if = "is_nan_f32")]
	pub se:f32,
}
impl TempHumBaroData {
	pub fn new() -> Self {
		TempHumBaroData{
			d:"".to_string(),
			hu:"".to_string(),
			te:f32::NAN,
			tm:f32::NAN,
			ta:f32::NAN,
			ba:"".to_string(),
			ch:f32::NAN,
			se:f32::NAN,
		}
	}
}