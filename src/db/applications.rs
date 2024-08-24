use std::{collections::HashMap, error::Error};
use domorust_models::applications::Application;
use domorust_models::{FromHashMap, FromSqlRow, ToSqlQuery};
use rusqlite::Connection;

pub fn get_applications(_params: HashMap<String, String>) -> Result<Vec<Application>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "SELECT * FROM Applications";
	let mut stmt = connection.prepare(query)?;
	let apps_iter = stmt.query_map((), |row| {
		Application::get_from_row(row)
	})?;
	let mut res=vec![];
	for h in apps_iter {
		res.push(h?);
	}
	Ok(res)
}

pub fn add_application(fields: HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let res=Application::from_hashmap(&fields)?;
	res.add_query(&connection)?;
	Ok(())
}
pub fn update_application(_idx: usize,fields: HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	// TODO: Merge with database
	let res=Application::from_hashmap(&fields)?;
	res.add_query(&connection)?;
	Ok(())
}

pub fn delete_application(idx: usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "DELETE FROM Applications WHERE ID==?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}