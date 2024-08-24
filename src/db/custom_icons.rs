use std::{collections::HashMap, error::Error};

use domorust_models::custom_images::CustomIcon;
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};

use rusqlite::Connection;


pub fn get_custom_icons(params: HashMap<String, String>) -> Result<Vec<CustomIcon>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut res=CustomIcon::get_items_from_table(&connection, &params)?;
	for img in &mut res {
		img.IconFile16 = format!("domorust-api/custom_icons/{}/image_small", img.ID);
		img.IconFile48Off = format!("domorust-api/custom_icons/{}/image_off", img.ID);
		img.IconFile48On = format!("domorust-api/custom_icons/{}/image_on", img.ID);
	}
	Ok(res)
}
pub fn get_custom_icon(id: usize) -> Result<CustomIcon, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut img=CustomIcon::get_item_from_table(&connection, id)?;
	img.IconFile16 = format!("domorust-api/custom_icons/{}/image_small", img.ID);
	img.IconFile48Off = format!("domorust-api/custom_icons/{}/image_off", img.ID);
	img.IconFile48On = format!("domorust-api/custom_icons/{}/image_on", img.ID);
	Ok(img)
}
pub fn get_custom_icons_small_image(idx: usize) -> Result<Vec<u8>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut stmt = connection.prepare("SELECT IconSmall FROM CustomImages WHERE ID=?1")?;
	let res=stmt.query_row([idx], |row| {
		row.get::<usize,Vec<u8>>(0)
	});
	Ok(res?)
}
pub fn get_custom_icons_on_image(idx: usize) -> Result<Vec<u8>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut stmt = connection.prepare("SELECT IconOn FROM CustomImages WHERE ID=?1")?;
	let res=stmt.query_row([idx], |row| {
		row.get::<usize,Vec<u8>>(0)
	});
	Ok(res?)
}
pub fn get_custom_icons_off_image(idx: usize) -> Result<Vec<u8>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut stmt = connection.prepare("SELECT IconOff FROM CustomImages WHERE ID=?1")?;
	let res=stmt.query_row([idx], |row| {
		row.get::<usize,Vec<u8>>(0)
	});
	Ok(res?)
}
pub fn add_custom_icon(params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	let var=CustomIcon::from_hashmap(&params)?;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_custom_icon(idx:usize, params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	//TODO: find a way to not clone the map
	let mut params=params.clone();
	params.insert("idx".to_string(), idx.to_string());
	let var=CustomIcon::from_hashmap(&params)?;
	//TODO: merge with database values
	var.update_query(&connection)?;
	Ok(())
}

pub fn delete_custom_icon(idx: usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "DELETE FROM CustomImages WHERE ID==?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}


