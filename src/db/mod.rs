#![allow(non_snake_case)]
pub(crate) mod applications;
pub(crate) mod create;
pub(crate) mod custom_icons;
pub(crate) mod devices;
pub(crate) mod events_scripts;
pub(crate) mod hardwares;
pub(crate) mod plans;
pub(crate) mod scenes;
pub(crate) mod settings;
pub(crate) mod timers;
pub(crate) mod user_variables;
pub(crate) mod users;

use std::error::Error;

use chrono::{Datelike, Duration, Local};
use create::{create_tables_if_needed, migrate_from_domoticz};
use domorust_models::{device::TempHumBaroData, settings::{ConfigResponseSettings, GPSCoord}, FromSqlRowFields};
use rusqlite::{Connection, Result};


/*pub trait ToSqlQueries : Sized {
	fn select(connection:&rusqlite::Connection, params: &HashMap<String,String>) -> Result<Vec<Self>, rusqlite::Error>;
	fn add(&self, connection:&rusqlite::Connection, params: &HashMap<String,String>) -> Result<(), rusqlite::Error>;
	fn update(&self, connection:&rusqlite::Connection, params: &HashMap<String,String>) -> Result<(), rusqlite::Error>;
}*/



/// Initializes the data store
///
/// Returns a Db type that either contains customer data
/// or is empty.
pub fn init_db() {
	migrate_from_domoticz().unwrap();
	
	// panics if error
	create_tables_if_needed().unwrap();
}

pub fn get_graph_data(idx: usize, sensor:&String, range:&String) -> Result<Vec<TempHumBaroData>, rusqlite::Error> {
	let mut res : Vec<TempHumBaroData> = vec![];
	//let devices = get_devices_db().unwrap();
	//let device = devices.iter().find(|d| {d.ID == idx.to_string()}).unwrap();
	let connection = Connection::open("domorust.db").unwrap();
	//let query = format!("SELECT Type, SubType, SwitchType, AddjValue, AddjMulti, AddjValue2, Options FROM DeviceStatus WHERE (ID == {} )", idx);
	
	// prix
	//let query = format!("SELECT strftime('%%Y-%%m-%%d %%H:00:00', Date) as ymd, MIN(Value1) as u1, MIN(Value5) as u2, MIN(Value2) as d1, MIN(Value6) as d2, Price FROM {} WHERE (DeviceRowID=={}) GROUP BY ymd",
	//						"Temperature_Calendar", idx);
	if sensor == "temp" {
		if range == "day" {
			let query = format!("SELECT Temperature, Chill, Humidity, Barometer, Date, SetPoint FROM {} WHERE (DeviceRowID=={}) ORDER BY Date ASC",
				"Temperature", idx);
			let mut stmt = connection.prepare(query.as_str())?;
			let mut devices_rows = stmt.query([])?;
			while let Ok(Some(row)) = devices_rows.next() {
				let mut datum = TempHumBaroData::new();
				datum.d = row.get(4)?;
				datum.te = row.get(0).unwrap_or(f32::NAN);
				let hum = row.get::<usize,u8>(2).unwrap_or(u8::MAX);
				if hum != u8::MAX {
					datum.hu = hum.to_string();
				}
				res.push(datum);
			}
		} else {
			let now=Local::now();
			let date_end=format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());
			let date_start= if range == "year" {
				let begin = now - Duration::days(365);
				format!("{:04}-{:02}-{:02}", begin.year(), begin.month(), begin.day())
			} else {
				let begin = now - Duration::days(30);
				format!("{:04}-{:02}-{:02}", begin.year(), begin.month(), begin.day())
			};

			let query = format!("SELECT Temp_Min, Temp_Max, Chill_Min, Chill_Max, Humidity, Barometer, Temp_Avg, Date, SetPoint_Min, SetPoint_Max, SetPoint_Avg FROM {} WHERE (DeviceRowID=={} AND Date>='{}' AND Date<='{}') ORDER BY Date ASC",
				"Temperature_Calendar", idx, date_start, date_end);
			let mut stmt = connection.prepare(query.as_str())?;
			let mut devices_rows = stmt.query([])?;
			while let Ok(Some(row)) = devices_rows.next() {
				let mut datum = TempHumBaroData::new();
				datum.d = row.get(7)?;
				datum.te = row.get(1)?;
				datum.tm = row.get(0)?;
				datum.ta = row.get(6)?;
				res.push(datum);
			}
		}
	}
	Ok(res)
}


pub fn get_latitude_longitude() -> Result<GPSCoord, Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "SELECT sValue FROM Preferences WHERE Key='Location'";
	let res = connection.query_row_and_then(query, [],|row| {
		let loc:GPSCoord=row.get(0)?;
		Ok::<GPSCoord, Box<dyn Error>>(loc)
	})?;
	Ok(res)
}

pub fn get_config() -> Result<ConfigResponseSettings, Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	ConfigResponseSettings::read(&connection)
}