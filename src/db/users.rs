use std::{collections::HashMap, error::Error};

use domorust_models::users::User;
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_users(params: &HashMap<String, String>) -> Result<Vec<User>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=User::build_from_table(&connection, params)?;
	Ok(res)
}
pub fn add_user(params:&HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let var=User::from_hashmap(params)?;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_user(idx:usize, params:&HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	//TODO: find a way to not clone the map
	let mut params=params.clone();
	params.insert("idx".to_string(), idx.to_string());
	let var=User::from_hashmap(&params)?;
	//TODO: merge with database values
	var.update_query(&connection)?;
	Ok(())
}

pub fn delete_user(idx: usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "DELETE FROM Users WHERE ID=?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}
/*pub fn get_session(id:&str) -> Result<UserSession, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "SELECT * FROM UserSession WHERE SessionID=?1";
	connection.query_row(query, [id], |row|{
		UserSession::build_from_row(row)
	}).map_err(|e| e.into())
}*/