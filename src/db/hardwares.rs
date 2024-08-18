use std::collections::HashMap;

use domorust_models::hardware::HardwareData;
use rusqlite::{Connection, Result};


pub fn get_hardwares_data(_params: HashMap<String, String>) -> Result<Vec<HardwareData>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "SELECT * FROM Hardware";
	let mut hardwares: Vec<HardwareData> = vec![];
	let mut stmt = connection.prepare(query)?;
	let hardwares_iter = stmt.query_map([], |row| {
		Ok(HardwareData {
			ID: row.get(0)?,
			Name: row.get(1)?,
			Enabled: row.get(2)?,
			Type: row.get(3)?,
			Address: row.get(4)?,
			Port: row.get(5)?,
			SerialPort: row.get(6)?,
			Username: row.get(7)?,
			Password: row.get(8)?,
			Extra: row.get(9)?,
			Mode1: row.get(10)?,
			Mode2: row.get(11)?,
			Mode3: row.get(12)?,
			Mode4: row.get(13)?,
			Mode5: row.get(14)?,
			Mode6: row.get(15)?,
			DataTimeout: row.get(16)?,
			Configuration: row.get(17)?,
			LogLevel: row.get(18)?,
		})
	})?;
	for h in hardwares_iter.flatten() {
		hardwares.push(h);
	}
	Ok(hardwares)
}
pub fn get_hardware_data(idx:usize) -> Result<HardwareData> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "SELECT * FROM Hardware WHERE ID=?1";
	let mut stmt = connection.prepare(query)?;
	
	stmt.query_row([idx], |row| {
		Ok(HardwareData {
			ID: row.get(0)?,
			Name: row.get(1)?,
			Enabled: row.get(2)?,
			Type: row.get(3)?,
			Address: row.get(4)?,
			Port: row.get(5)?,
			SerialPort: row.get(6)?,
			Username: row.get(7)?,
			Password: row.get(8)?,
			Extra: row.get(9)?,
			Mode1: row.get(10)?,
			Mode2: row.get(11)?,
			Mode3: row.get(12)?,
			Mode4: row.get(13)?,
			Mode5: row.get(14)?,
			Mode6: row.get(15)?,
			DataTimeout: row.get(16)?,
			Configuration: row.get(17)?,
			LogLevel: row.get(18)?,
		})
	})
}

