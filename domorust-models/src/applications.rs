
#![allow(non_snake_case)]

use chrono::NaiveDateTime;
use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use serde::Serialize;

use crate::{FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery};

#[derive(Debug, Clone, Default, Serialize)]
#[derive(FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery)]
#[table_name("Applications")]
pub struct Application {
	#[primary_key]
	#[serde(alias="idx", with="crate::utils::string")]
	pub ID: usize,
	pub Active: bool,
	pub Public: bool,
	pub Applicationname: String,
	pub Secret: String,
	pub Pemfile:String,
	pub LastSeen: NaiveDateTime,
	pub LastUpdate: NaiveDateTime
}