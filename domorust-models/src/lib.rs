use std::{collections::HashMap, error::Error, fmt::Display};

use rusqlite::Connection;
use serde::Serialize;
pub mod applications;
pub mod connection;
pub mod custom_images;
pub mod device;
pub mod domorust;
pub mod event_scripts;
pub mod hardware;
pub mod plans;
pub mod plugins;
pub mod scenes;
pub mod settings;
pub mod timers;
pub mod users;
pub mod user_variables;
pub mod utils;



#[derive(Debug)]
pub struct ParseDomorustTypeError {
	name:&'static str,
	input: String
}
impl ParseDomorustTypeError {
	pub fn new(name:&'static str, input: String) -> Self {
		Self{name, input}
	}
}
impl std::error::Error for ParseDomorustTypeError{}
impl Display for ParseDomorustTypeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Enable to parse {} from str \"{}\"", self.name, self.input)
	}
}


pub trait FromSqlRowFields : Sized {
	fn read(connection:&Connection) -> Result<Self, Box<dyn std::error::Error>>;
	fn get_column_name(field_name:&String) -> Result<String, Box<dyn std::error::Error>>;
}
pub trait ToSqlRowFields : Sized {
	fn write(connection:&Connection, params:HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>>;
	fn write_instance(&self, connection:&Connection) -> Result<(), Box<dyn std::error::Error>>;
}
pub trait FromSqlRow : Sized {
	fn build_from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
}
pub trait FromSqlTable : FromSqlRow {
	fn build_from_table(connection:&rusqlite::Connection, filters:&HashMap<String,String>) -> Result<Vec<Self>, rusqlite::Error>;
}
pub trait ToSqlQuery {
	fn add_query(&self, connection:&rusqlite::Connection) -> Result<(), rusqlite::Error>;
	fn update_query(&self, connection:&rusqlite::Connection) -> Result<(), rusqlite::Error>;
}
pub trait FromHashMap : Sized {
	fn from_hashmap(params: &HashMap<String,String>) -> Result<Self, Box<dyn Error>>;
}


#[derive(Debug)]
pub struct ParseTypeError {
	name:&'static str,
	reason:&'static str
}
impl ParseTypeError {
	pub fn new(name:&'static str, reason:&'static str) -> Self {
		Self { name, reason }
	}
}
impl std::error::Error for ParseTypeError{}
impl Display for ParseTypeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Enable to parse {}: {}", self.name, self.reason)
	}
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UptimeData {
	pub days:u16,
	pub hours:u8,
	pub minutes:u8,
	pub seconds:u8,
}