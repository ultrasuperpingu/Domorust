#![allow(non_snake_case)]

use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use crate::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
#[derive(FromSqlTable, FromSqlRow, FromHashMap, ToSqlQuery)]
#[table_name("Plans")]
pub struct Plan
{
	#[primary_key]
	#[serde(rename="idx",with="crate::utils::string")]
	#[param_name("idx")]
	pub ID: usize,// : "3"
	pub Name: String,//" : "Salon",
	//#[param_name("FloorPlanID")]
	pub FloorPlanID: usize,
	pub Area: String,
	#[skip_field]
	pub Devices: usize,//" : 16,
	#[serde(with="crate::utils::string")]
	pub Order: usize,// : "3",
}

#[derive(Debug, Default, Clone, Serialize)]
#[derive(FromSqlTable, FromSqlRow, FromHashMap)]
#[table_name("FloorPlans")]
pub struct FloorPlan
{
	#[serde(rename="idx", with="crate::utils::string")]
	pub ID: usize,//" : "1"
	pub Name: String,//" : "Maison",
	#[skip_field]
	pub Image: String,//" : "images/floorplans/plan?idx=1",
	#[serde(with="crate::utils::string")]
	pub Order: usize,//" : "1",
	#[skip_field]
	pub Plans: usize,//" : 6,
	#[serde(with="crate::utils::string")]
	pub ScaleFactor: f32,//" : "1.0",
}
