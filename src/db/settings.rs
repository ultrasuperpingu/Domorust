use std::collections::HashMap;
use std::error::Error;

use domorust_models::settings::{Settings, FloorPlansConfig};
use domorust_models::{FromSqlRowFields, ToSqlRowFields};



pub fn get_settings(_params: HashMap<String, String>) -> Result<Settings, Box<dyn Error>> {
	let connection = rusqlite::Connection::open("domorust.db")?;
	Settings::read(&connection)
}

pub fn store_settings(params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	println!("{:?}", params);
	let connection = rusqlite::Connection::open("domorust.db")?;
	Settings::write(&connection, params)
}

pub fn get_setting_int(name:&str) -> Result<i64, Box<dyn Error>> {
	let connection = rusqlite::Connection::open("domorust.db")?;
	let query = "SELECT nValue FROM Preferences WHERE Key=?1";
	let res: Result<i64, rusqlite::Error> = connection.query_row(query, [name], |row| {
		row.get(0)
	});
	Ok(res?)
}

pub fn get_setting_string(name:&str) -> Result<String, Box<dyn Error>> {
	let connection = rusqlite::Connection::open("domorust.db")?;
	let query = "SELECT sValue FROM Preferences WHERE Key=?1";
	let res: Result<String, rusqlite::Error> = connection.query_row(query, [name], |row| {
		row.get(0)
	});
	Ok(res?)
}
pub fn set_setting(name:&str, val:&String) -> Result<(), Box<dyn Error>> {
	if Settings::get_column_name(&name.to_string())? == "nValue" {
		set_setting_int(name, val.parse()?)
	} else {
		set_setting_string(name, val)
	}
}
pub fn set_setting_int(name:&str, val: i32) -> Result<(), Box<dyn Error>> {
	let connection = rusqlite::Connection::open("domorust.db")?;
	let query = "UPDATE Preferences SET nValue VALUES ?1 WHERE Key=?2";
	let res = connection.execute(query, (name, val))?;
	if res != 1 {
		return Err(format!("Expected to Update exactly one row. update : {}", res).into());
	}
	Ok(())
}

pub fn set_setting_string(name:&str, val:&String) -> Result<(), Box<dyn Error>> {
	let connection = rusqlite::Connection::open("domorust.db")?;
	let query = "UPDATE Preferences SET sValue VALUES ?1 WHERE Key=?2";
	let res = connection.execute(query, (name, val))?;
	if res != 1 {
		return Err(format!("Expected to Update exactly one row. update : {}", res).into());
	}
	Ok(())
}

pub fn get_floorplans_settings() -> Result<FloorPlansConfig, Box<dyn Error>> {
	let connection = rusqlite::Connection::open("domorust.db")?;
	FloorPlansConfig::read(&connection)
}