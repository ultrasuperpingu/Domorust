use std::{collections::HashMap, error::Error};

use domorust_models::event_scripts::{DeviceCurrentStatus, EventsScript};
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_events_scripts(params: HashMap<String, String>) -> Result<Vec<EventsScript>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=EventsScript::build_from_table(&connection, &params)?;
	Ok(res)
}
pub fn add_events_script(params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	let var=EventsScript::from_hashmap(&params)?;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_events_script(idx:usize, params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	//TODO: find a way to not clone the map
	let mut params=params.clone();
	params.insert("idx".to_string(), idx.to_string());
	let var=EventsScript::from_hashmap(&params)?;
	//TODO: merge with database values
	var.update_query(&connection)?;
	Ok(())
}

pub fn delete_events_script(idx: usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "DELETE FROM EventMaster WHERE ID==?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}
pub fn get_devices_current_status(params:HashMap<String,String>) -> Result<Vec<DeviceCurrentStatus>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut res=DeviceCurrentStatus::build_from_table(&connection, &params)?;
	for r in &mut res {
		r.Values = r.nValue.to_string() + "/" + r.Value.as_str();
	}
	Ok(res)
}
