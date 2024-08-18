#![allow(non_snake_case)]
use chrono::NaiveDate;
use serde::Serialize;

use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};

use crate::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};

#[derive(Debug, Serialize, Default)]
#[derive(FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery)]
#[table_name("Users")]
pub struct User {
	#[serde(rename(serialize = "idx"), with="crate::utils::string")]
	#[primary_key]
	#[param_name("idx")]
	pub ID: usize,
	#[serde(rename(serialize = "Enabled"), with="crate::utils::string")]
	#[param_name("enabled")]
	pub Active:bool,
	#[param_name("username")]
	#[serde(with="crate::utils::base64_decoded")]
	pub Username:String,
	#[param_name("password")]
	pub Password:String,
	#[param_name("rights")]
	pub Rights: u16,
	pub TabsEnabled: u8,
	pub RemoteSharing: bool,
	#[serde(skip)]
	#[skip_field]
	#[allow(unused)]
	pub MFAsecret:String,
}
#[derive(Debug, Serialize, Default, FromSqlRow, ToSqlQuery)]
pub struct UserSession {
	#[primary_key]
	pub SessionID:String,
	pub Username:String,
	pub AuthToken:String,
	pub ExpirationDate:NaiveDate,
	pub RemoteHost:String,
	pub LastUpdate: NaiveDate
}