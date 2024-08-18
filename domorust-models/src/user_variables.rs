#![allow(non_snake_case)]
use chrono::NaiveDateTime;
use domorust_macros::{FromSqlRow, FromSqlTable, ToSqlQuery, FromHashMap};
use serde::Serialize;

use crate::{FromSqlTable, FromSqlRow, ToSqlQuery, FromHashMap};
#[derive(Serialize, Clone, Default, Debug)]
#[derive(FromSqlTable, FromSqlRow, ToSqlQuery, FromHashMap)]
#[table_name("UserVariables")]
#[allow(non_snake_case)]
pub struct UserVariable {
	#[serde(rename="idx", with="crate::utils::string")]
	#[param_name("idx")]
	#[primary_key]
	pub ID: usize,
	#[param_name("vname")]
	pub Name: String,
	#[serde(rename="Type", with="crate::utils::string")]
	#[param_name("vtype")]
	pub ValueType: usize,
	#[param_name("vvalue")]
	pub Value: String,
	pub LastUpdate: NaiveDateTime
}
