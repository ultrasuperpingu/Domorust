use std::str::FromStr;

use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use serde::{Deserialize, Serialize};

use crate::ParseDomorustTypeError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
	r:u8,
	g:u8,
	b:u8,
	cw:u8,
	m:u8,
	t:u8,
	ww:u8
}
impl FromSql for Color {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let res=serde_json::from_str::<Color>(value.as_str()?).map_err(|_e| FromSqlError::InvalidType);
		res
	}
}
impl ToSql for Color {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		let res=serde_json::ser::to_string(self);
		let res=res.map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
		Ok(rusqlite::types::ToSqlOutput::Owned(res.into()))
	}
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum BasicData {
	Bool(bool),
	Int(i64),
	Float(f64),
	String(String),
	Color(Color)
}
impl Default for BasicData {
	fn default() -> Self {
		BasicData::Bool(false)
	}
}
impl BasicData {
	pub fn is_bool(&self) -> bool {
		matches!(self, BasicData::Bool(_))
	}
	pub fn is_int(&self) -> bool {
		matches!(self, BasicData::Int(_))
	}
	pub fn is_float(&self) -> bool {
		matches!(self, BasicData::Float(_))
	}
	pub fn is_string(&self) -> bool {
		matches!(self, BasicData::String(_))
	}
	pub fn is_color(&self) -> bool {
		matches!(self, BasicData::Color(_))
	}
	pub fn bool(&self) -> Option<bool> {
		if let BasicData::Bool(res) = self {
			Some(*res)
		} else {
			None
		}
	}
	pub fn int(&self) -> Option<i64> {
		if let BasicData::Int(res) = self {
			Some(*res)
		} else {
			None
		}
	}
	pub fn float(&self) -> Option<f64> {
		if let BasicData::Float(res) = self {
			Some(*res)
		} else {
			None
		}
	}
	pub fn string(&self) -> Option<&String> {
		if let BasicData::String(res) = self {
			Some(res)
		} else {
			None
		}
	}
	pub fn color(&self) -> Option<&Color> {
		if let BasicData::Color(res) = self {
			Some(res)
		} else {
			None
		}
	}
}

impl ToSql for BasicData {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		let res=serde_json::ser::to_string(self);
		let res=res.map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
		Ok(rusqlite::types::ToSqlOutput::Owned(res.into()))
	}
}

impl FromStr for BasicData {
	type Err = ParseDomorustTypeError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		//TODO: redesign this. How produce a string containing a float for example
		let res=serde_json::from_str::<Color>(value);
		if res.is_ok() {
			return res.map(|v| BasicData::Color(v)).map_err(|e| ParseDomorustTypeError::new("BasicData", value.to_string() + " caused by " + e.to_string().as_str()));
		}
		let res=value.parse::<f64>();
		if let Ok(res) = res {
			return Ok(BasicData::Float(res));
		}
		let res=value.parse::<i64>();
		if let Ok(res) = res {
			return Ok(BasicData::Int(res));
		}
		Ok(BasicData::String(String::from(value)))
		//Err(ParseDomorustTypeError::new("BasicData", value.to_string()))
	}
}