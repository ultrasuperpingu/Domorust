use std::collections::HashMap;

use domorust_models::users::User;
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_users(params: &HashMap<String, String>) -> Result<Vec<User>, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let res=User::get_items_from_table(&connection, params)?;
	Ok(res)
}
pub fn get_user(id: usize) -> Result<User, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let res=User::get_item_from_table(&connection, id)?;
	Ok(res)
}
pub fn add_user(params:&HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>> {
	let connection = Connection::open("domorust.db")?;
	let var=User::from_hashmap(params)?;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_user(id:usize, params:&HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>> {
	let connection = Connection::open("domorust.db")?;
	//TODO: find a way to not clone the map
	let mut params=params.clone();
	params.insert("idx".to_string(), id.to_string());
	//TODO: do not read in db to merge, just make the update on provided fields
	let mut u=get_user(id)?;
	u.update_from_hashmap(&params)?;
	u.update_query(&connection)?;
	Ok(())
}

pub fn delete_user(idx: usize) -> Result<(), rusqlite::Error> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "DELETE FROM Users WHERE ID=?1";
	let _ = connection.execute(query, (idx,))?;
	Ok(())
}
/*pub fn get_session(id:&str) -> Result<UserSession, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let query = "SELECT * FROM UserSession WHERE SessionID=?1";
	connection.query_row(query, [id], |row|{
		UserSession::build_from_row(row)
	}).map_err(|e| e.into())
}*/