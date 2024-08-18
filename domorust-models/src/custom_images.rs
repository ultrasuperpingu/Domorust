#![allow(non_snake_case)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::LazyLock;

use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use serde::Serialize;

use crate::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};


#[derive(Debug, Clone, Default, Serialize)]
#[derive(FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery)]
#[table_name("CustomImages")]
pub struct CustomIcon
{
	#[primary_key]
	#[serde(rename="idx")]
	pub ID: usize, // : 3
	#[column_name("Name")]
	pub Title:String,         // : "D2L Electricity Meter",
	#[serde(skip)]
	pub Base:String,
	pub Description:String, // : "D2L Electricity Meter",
	//#[column_name("IconSmall")]
	#[skip_field]
	pub IconFile16:String,  // : "images/D2LModuleElecMeter.png",
	//#[column_name("IconOff")]
	#[skip_field]
	pub IconFile48Off:String, // : "images/D2LModuleElecMeter48_Off.png",
	#[skip_field]
	//#[column_name("IconOn")]
	pub IconFile48On:String,  // : "images/D2LModuleElecMeter48_On.png",
}
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeviceImage
{
	pub description:String,// : "Air Conditioner/HVAC",
	pub idx: usize,// : 23,
	pub imageSrc:String,//" : "AC",
	pub text:String//" : "Air Conditioner/HVAC"
}
static CUSTOM_IMAGES : LazyLock<HashMap<usize,DeviceImage>> = LazyLock::new(|| {
	let mut res=HashMap::new();
	let mut index = 0usize;
	//TODO: replace www with webroot
	if let Ok(lines) = read_lines("www/switch_icons.txt") {
		// Consumes the iterator, returns an (Optional) String
		for line in lines.flatten() {
			let fields = line.split(';');
			let fields = fields.collect::<Vec<&str>>();
			if fields.len() == 3 {
				res.insert(index, DeviceImage {
					idx: index,
					imageSrc:fields[0].to_string(),
					text: fields[1].to_string(),
					description: fields[2].to_string()
					}
				);
				index+=1;
			}
		}
	}
	res
});
pub fn get_images() -> Vec<DeviceImage> {
	let images = CUSTOM_IMAGES.values().cloned().collect();
	//TODO: add custom icons
	images
}
pub fn get_image(index:usize) -> Option<&'static DeviceImage> {
	CUSTOM_IMAGES.get(&index)
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}