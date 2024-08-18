#![allow(non_snake_case)]

use std::{fmt::Display, str::FromStr};

use chrono::NaiveDateTime;
use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use serde::Serialize;

use crate::{device::NameValueList, FromHashMap, FromSqlRow, FromSqlTable, ParseTypeError, ToSqlQuery};

#[derive(Clone, Debug, Default, Serialize, Copy)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum SceneType{
	#[default]
	Scene = 0,
	Group = 1,
}
impl Display for SceneType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}
impl FromStr for SceneType {

	type Err = ParseTypeError;

	fn from_str(input: &str) -> Result<SceneType, Self::Err> {
		match input {
			"Scene"  => Ok(SceneType::Scene),
			"Group"  => Ok(SceneType::Group),
			_      => Err(ParseTypeError::new("SceneType","Invalid string value")),
		}
	}
}
impl FromSql for SceneType {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val=u8::try_from(value.as_i64()?).map_err(|e|{FromSqlError::Other(Box::new(e))})?;
		SceneType::from_u8(val)
			.ok_or(FromSqlError::Other(Box::new(ParseTypeError::new("SceneType","Invalid u8 value"))))
	}
}
impl ToSql for SceneType {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok((*self as u8).into())
	}
}


#[derive(Debug, Serialize, Default, Clone)]
#[derive(FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery)]
#[table_name("Scenes")]
pub struct Scene
{
	#[serde(rename="idx",with="crate::utils::string")]
	#[primary_key]
	#[param_name("idx")]
	ID:usize, // : "2"
	#[param_name("name")]
	Name:String, // : "fdf",
	#[serde(skip)]
	Activators:Option<NameValueList>,
	Favorite:u8,// : 0,
	Order: usize,
	#[param_name("description")]
	Description:String,// : "",
	LastUpdate:NaiveDateTime,// : "2024-08-02 10:07:59",
	#[param_name("offaction")]
	OffAction:String,// : "",
	#[param_name("onaction")]
	OnAction:String,// : "",
	#[param_name("protected")]
	Protected:bool,// : false,
	#[skip_field]
	Status:String,// : "Off",
	#[skip_field]
	#[serde(with="crate::utils::string")]
	Timers:bool, // : "false",
	#[column_name("SceneType")]
	#[param_name("scenetype")]
	//#[serde(with="crate::utils::string")]
	Type:SceneType, // : "Scene",
	#[skip_field]
	UsedByCamera:bool,// : false,
}
#[derive(Debug, Serialize, Default)]
#[derive(FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery)]
#[table_name("SceneDevices")]
pub struct SceneDevice
{
	#[primary_key]
	#[serde(with="crate::utils::string")]
	ID:usize,//" : "1",
	Color:String,//" : "{\"b\":0,\"cw\":60,\"g\":0,\"m\":2,\"r\":0,\"t\":195,\"ww\":195}",
	#[column_name("Cmd")]
	//#[serde(with="crate::utils::string")]
	Command:Command,//" : "Off",
	#[column_name("DeviceRowID")]
	#[serde(with="crate::utils::string")]
	DevID:usize,//" : "36",
	#[column_name("DeviceRowID")]
	#[serde(with="crate::utils::string")]
	DevRealIdx:usize,//" : "36",
	#[param_name("idx")]
	//#[serde(with="crate::utils::string")]
	SceneRowID:usize,// added
	Level:u8,//" : 1,
	#[skip_field]
	Name:String,//" : "Salon C",
	OffDelay:u32,//" : 0,
	OnDelay:u32,//" : 0,
	Order:usize,//" : 1,
	#[skip_field]
	SubType:String,//" : "RGBWW",
	#[skip_field]
	Type:String,//" : "Color Switch"
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SceneActivation
{
	idx:usize, //" : 183,
	name: String,//" : "Telco Volet Salon"
	code: u8,// : 1,
	codestr:String,//" : "On",
}

#[derive(Clone, Debug, Default, Serialize, Copy)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum Command{
	#[default]
	On = 0,
	Off,
	Open,
	Close,
	Stop,
	Switch
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl FromStr for Command {

	type Err = ParseTypeError;

	fn from_str(input: &str) -> Result<Command, Self::Err> {
		match input {
			"On"  => Ok(Command::On),
			"Off"  => Ok(Command::Off),
			"Open"  => Ok(Command::Open),
			"Close"  => Ok(Command::Close),
			"Stop"  => Ok(Command::Stop),
			"Switch"  => Ok(Command::Switch),
			_      => Err(ParseTypeError::new("Command","Invalid string value")),
		}
	}
}

impl FromSql for Command {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val=u8::try_from(value.as_i64()?).map_err(|e|{FromSqlError::Other(Box::new(e))})?;
		Command::from_u8(val)
			.ok_or(FromSqlError::Other(Box::new(ParseTypeError::new("Command","Invalid u8 value"))))
	}
}

impl ToSql for Command {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok((*self as u8).into())
	}
}
