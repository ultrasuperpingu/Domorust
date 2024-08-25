use std::{collections::HashMap, error::Error};

use domorust_models::event_scripts::{DeviceCurrentStatus, EventsScript};
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_events_scripts(params: HashMap<String, String>) -> Result<Vec<EventsScript>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=EventsScript::get_items_from_table(&connection, &params)?;
	Ok(res)
}
pub fn get_events_script(id: usize) -> Result<EventsScript, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=EventsScript::get_item_from_table(&connection, id)?;
	Ok(res)
}
pub fn add_events_script(params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	let var=EventsScript::from_hashmap(&params)?;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_events_script(id:usize, params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	//TODO: do not read in db to merge, just make the update on provided fields
	let mut a=get_events_script(id)?;
	a.update_from_hashmap(&params)?;
	a.update_query(&connection)?;
	
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
	let mut res=DeviceCurrentStatus::get_items_from_table(&connection, &params)?;
	for r in &mut res {
		r.Values = r.nValue.to_string() + "/" + r.Value.as_str();
	}
	Ok(res)
}
