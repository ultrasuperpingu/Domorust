#![allow(non_snake_case)]
use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use serde::Serialize;

use crate::{FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery};

#[derive(Debug, Clone, Default, Serialize)]
#[derive(FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery)]
#[table_name("EventMaster")]
pub struct EventsScript
{
	#[primary_key]
	#[serde(rename="id", with="crate::utils::string")]
	pub ID: usize,
	#[serde(rename="name")]
	pub Name:String,
	#[serde(rename="interpreter", skip_serializing_if="String::is_empty")]	
	pub Interpreter:String,
	#[serde(rename="type", skip_serializing_if="String::is_empty")]	
	pub Type:String,
	#[serde(rename="xmlstatement", skip_serializing_if="String::is_empty")]	
	pub XMLStatement:String,
	#[serde(rename="eventstatus", with="crate::utils::string")]	
	pub Status:u8,
}

#[derive(Debug, Clone, Default, Serialize)]
#[derive(FromSqlRow, FromSqlTable)]
#[table_name("DeviceStatus")]
pub struct DeviceCurrentStatus
{
	#[serde(rename="id")]	
	pub ID : usize,
	#[serde(rename="lastupdate")]	
	pub LastUpdate: String,
	#[serde(rename="name")]	
	pub Name:String,
	#[serde(skip)]	
	pub nValue:i32,
	#[column_name("sValue")]
	#[serde(rename="value")]	
	pub Value:String,
	#[skip_field]
	#[serde(rename="values")]	
	pub Values:String
}
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeviceImage
{
	pub description:String,// : "Air Conditioner/HVAC",
	pub idx: usize,// : 23,
	pub imageSrc:String,//" : "AC",
	pub text:String//" : "Air Conditioner/HVAC"
}