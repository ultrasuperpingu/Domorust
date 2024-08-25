use std::{collections::HashMap, error::Error};
use domorust_models::applications::Application;
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_applications(filters: HashMap<String, String>) -> Result<Vec<Application>, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	Application::get_items_from_table(&connection, &filters)
}
pub fn get_application(id: usize) -> Result<Application, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	Application::get_item_from_table(&connection, id)
}
pub fn add_application(fields: HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=Application::from_hashmap(&fields)?;
	res.add_query(&connection)?;
	Ok(())
}
pub fn update_application(id: usize,fields: HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	//TODO: do not read in db to merge, just make the update on provided fields
	let mut a=get_application(id)?;
	a.update_from_hashmap(&fields)?;
	a.update_query(&connection)?;
	Ok(())
}

pub fn delete_application(idx: usize) -> Result<(), rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let query = "DELETE FROM Applications WHERE ID==?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}