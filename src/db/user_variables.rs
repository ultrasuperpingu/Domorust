use std::{collections::HashMap, error::Error};

use domorust_models::user_variables::UserVariable;
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_user_variables(params: HashMap<String, String>) -> Result<Vec<UserVariable>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=UserVariable::get_items_from_table(&connection, &params)?;
	Ok(res)
}
pub fn get_user_variable(id: usize) -> Result<UserVariable, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=UserVariable::get_item_from_table(&connection, id)?;
	Ok(res)
}
pub fn add_user_variable(params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	let var=UserVariable::from_hashmap(&params)?;
	println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_user_variable(idx:usize, params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	//TODO: find a way to not clone the map
	let mut params=params.clone();
	params.insert("idx".to_string(), idx.to_string());
	let var=UserVariable::from_hashmap(&params)?;
	//TODO: merge with database values
	var.update_query(&connection)?;
	Ok(())
}

pub fn delete_user_variable(idx: usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "DELETE FROM UserVariables WHERE ID==?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}