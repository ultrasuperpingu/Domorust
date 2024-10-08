#![allow(non_snake_case)]

use chrono::NaiveDateTime;
use serde::Serialize;
use domorust_macros::{FromHashMap, FromSqlTable, ToSqlQuery};

use crate::{basic_data::BasicData, FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};


#[derive(Debug, Clone, Default, Serialize)]
#[derive(FromSqlTable, FromHashMap, ToSqlQuery)]
#[table_name("DevicesData")]
#[custom_select_columns("*")]
pub struct DeviceData {
	#[primary_key]
	pub ID: u32,
	pub DeviceID: u32,
	pub Name: String,
	pub Unit: String,
	#[serde(skip)]
	pub Type: u8,
	#[serde(skip)]
	pub HistoriseShort: bool,
	#[serde(skip)]
	pub Historise: bool,
	#[skip_field]
	pub Value: BasicData,
	pub LastUpdate: NaiveDateTime
}
impl FromSqlRow for DeviceData {
	fn get_from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
		let typ : u8 = row.get(4)?;
		let val = match typ {
			0 => {BasicData::Bool(row.get("BoolValue")?)},
			1 => {BasicData::Int(row.get("IntValue")?)},
			2 => {BasicData::Float(row.get("FloatValue")?)},
			3 => {BasicData::String(row.get("StringValue")?)},
			4 => {BasicData::Color(row.get("ColorValue")?)},
			v => {return Err(rusqlite::Error::IntegralValueOutOfRange(4, v as i64))}
		};
		Ok(DeviceData{
			ID: row.get(0)?,
			DeviceID: row.get(1)?,
			Name: row.get(2)?,
			Unit: row.get(3)?,
			Type: typ,
			HistoriseShort: row.get(5)?,
			Historise: row.get(6)?,
			Value: val,
			LastUpdate: row.get(12)?
		})
	}
}