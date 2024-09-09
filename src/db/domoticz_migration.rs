use std::{error::Error, path::Path};

use chrono::Utc;
use domorust_models::{settings::GPSCoord, utils::base64};
use rusqlite::Connection;
use sha2::{Digest, Sha256};

use crate::domoticz::consts::{DEVICE_SUBTYPES_DESC, DEVICE_TYPES_DESC};

use super::create::{sqlCreateDevicesData, sqlCreateDevicesDataHistory, sqlCreateUsers, DB_VERSION};


pub fn migrate_from_domoticz() -> Result<bool, Box<dyn Error>> {
	if !Path::new("domorust.db").exists() && Path::new("Domoticz.db").exists() {
		copy_and_upgrade_domoticz_db()?;
	} else {
		return Ok(false)
	}
	let connection=Connection::open("domorust.db")?;
	update_users(&connection)?;
	update_preferences_table(&connection)?;
	drop_useless_tables(&connection)?;
	
	update_devices(&connection)?;
	fill_device_data(&connection)?;
	
	connection.execute(sqlCreateDevicesDataHistory, [])?;
	let mut stmt = connection.prepare("SELECT DeviceRowID, Value, Counter, Date FROM Meter_Calendar")?;
	let _res = stmt.query_map([], |_row| {
		//TODO
		Ok(())
	})?;
	//for r in res.into_iter() {
	//	println!("{:?}", r);
	//}

	ensure_constraints(&connection, "Users", sqlCreateUsers)?;
	Ok(true)
}

fn fill_device_data(connection: &Connection) -> Result<(), Box<dyn Error>> {
	connection.execute(sqlCreateDevicesData, [])?;
	let mut stmt = connection.prepare("SELECT ID, nValue, sValue, Color, Type, SubType, LastUpdate FROM Devices")?;
	let res = stmt.query_map([], |row| {
			let id=row.get::<usize, i64>(0)?;
			let nVal=row.get::<usize, i64>(1)?;
			let sVal = row.get::<usize, Option<String>>(2)?;
		
			let cVal = row.get::<usize, Option<String>>(3)?;
			let typ=row.get::<usize, u8>(4)?;
			let styp=row.get::<usize, u8>(5)?;
			let lastUp = row.get::<usize, String>(6)?;
			let mut done=false;
			if let Some(c) = cVal {
				if !c.is_empty() {
					connection.execute("INSERT INTO DevicesData (Name, Type, ColorValue, DeviceID, LastUpdate) VALUES('Color', 4, ?1, ?2, ?3)", (c,id, &lastUp))?;
					done=true;
				}
			}
			if !done {
				let mut sv=String::new();
				if let Some(s) = sVal {
					sv = s;
				}

				let type_desc=DEVICE_TYPES_DESC.get(&(typ as u8)).unwrap_or(&("",""));
				if type_desc.0 == "Temp" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Temperature', 2, '°C', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Temp + Humidity" {
					let data=sv.split(';').collect::<Vec<&str>>();
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Temperature', 2, '°C', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Humidity', 2, '%', TRUE, ?1, ?2, ?3)", (data[1], id, &lastUp))?;
				}
				else if type_desc.0 == "Temp + Humidity + Baro" {
					let data=sv.split(';').collect::<Vec<&str>>();
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Temperature', 2, '°C', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Humidity', 1, '%', TRUE, ?1, ?2, ?3)", (data[1], id, &lastUp))?;
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Barometer', 1, 'hPa', TRUE, ?1, ?2, ?3)", (data[2], id, &lastUp))?;
				}
				else if type_desc.0 == "Air Quality" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Value', 1, '', TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "Barometer" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Barometer', 1, 'hPa', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
				}
				else if type_desc.0 == "Blinds" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Value', 1, '', TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "Chime" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Value', 1, '', TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "Color Switch" {
					//should be done
				}
				else if type_desc.0 == "Fan" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Speed', 1, 'rpm', TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "General" {
					let sub_type_desc=DEVICE_SUBTYPES_DESC.get(&(typ, styp));
					if let Some(sub_type_desc) = sub_type_desc {
						if *sub_type_desc == "kWh" && !sv.is_empty() {
							let data=sv.split(';').collect::<Vec<&str>>();
							connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Counter', 2, 'kWh', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
							connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Usage', 2, 'W', TRUE, ?1, ?2, ?3)", (data[1], id, &lastUp))?;
						} else {
							connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, StringValue, DeviceID, LastUpdate) VALUES ('Value', 3, ', FALSE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
						}
					}
					else {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, StringValue, DeviceID, LastUpdate) VALUES ('Value', 3, ', FALSE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Heating" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Value', '', 1, TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "Humidity" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Humidity', 1, '%', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
				}
				else if type_desc.0.starts_with("Lighting") {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Value', 1, '', TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "Lux" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Lux', 2, 'lux', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
				}
				else if type_desc.0 == "P1 Smart Meter" {
					let data=sv.split(';').collect::<Vec<&str>>();
					if data.len()>=6 {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Counter1', 1, 'Wh', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Counter2', 1, 'Wh', TRUE, ?1, ?2, ?3)", (data[1], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Counter3', 1, 'Wh', TRUE, ?1, ?2, ?3)", (data[2], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Counter4', 1, 'Wh', TRUE, ?1, ?2, ?3)", (data[3], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Usage1', 2, 'W', TRUE, ?1, ?2, ?3)", (data[4], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Usage2', 2, 'W', TRUE, ?1, ?2, ?3)", (data[5], id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Rain" {
					let data=sv.split(';').collect::<Vec<&str>>();
					if data.len()>=2 {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Rate', 2, 'mm/h', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Cumul', 2, 'mm', TRUE, ?1, ?2, ?3)", (data[1], id, &lastUp))?;
					}
				}
				else if type_desc.0.starts_with("Thermostat") {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('CommandTemperature', 2, '°C', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Usage" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Usage', 2, '??', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "UV" {
					let data=sv.split(';').collect::<Vec<&str>>();
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('UV', 2, 'UVI', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
				}
				else if type_desc.0 == "Gas" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Gas', 2, 'm^3', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Solar" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Solar', 2, '??', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Water" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Water', 2, 'm^3', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Water Level" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('WaterLevel', 2, 'm', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Weight" {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Weight', 2, 'kg', TRUE, ?1, ?2, ?3)", (nVal, id, &lastUp))?;
				}
				else if type_desc.0 == "Wind" {
					let data=sv.split(';').collect::<Vec<&str>>();
					if data.len() >= 4 {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('Angle', 2, '°', TRUE, ?1, ?2, ?3)", (data[0], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, StringValue, DeviceID, LastUpdate) VALUES ('Direction', 3, '', FALSE, ?1, ?2, ?3)", (data[1], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Speed', 1, 'm/s', TRUE, ?1, ?2, ?3)", (data[2], id, &lastUp))?;
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, IntValue, DeviceID, LastUpdate) VALUES ('Gust', 1, 'm/s', TRUE, ?1, ?2, ?3)", (data[3], id, &lastUp))?;
					}
				}
				else if type_desc.0 == "Setpoint" {
					if !sv.is_empty() {
						connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, FloatValue, DeviceID, LastUpdate) VALUES ('CommandTemperature', 2, '°C', TRUE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
					}
				}
				else if !sv.is_empty() {
					connection.execute("INSERT INTO DevicesData (Name, Type, Unit, Historise, StringValue, DeviceID, LastUpdate) VALUES ('Value', 3, '', FALSE, ?1, ?2, ?3)", (sv, id, &lastUp))?;
				}
			}
			Ok(())
		})?;
	Ok(for _r in res.into_iter() {
			//println!("{:?}", r);
		})
}

fn update_devices(connection: &Connection) -> Result<(), Box<dyn Error>> {
	connection.execute("ALTER TABLE DeviceStatus RENAME TO Devices;", [])?;
	connection.execute("ALTER TABLE Devices ADD WidgetType TEXT", [])?;
	connection.execute("UPDATE Devices SET WidgetType = 'generic'", [])?;
	connection.execute("ALTER TABLE Devices ADD Category TEXT", [])?;
	Ok(())
}

fn update_preferences_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
	connection.execute("ALTER TABLE Preferences ADD fValue REAL", [])?;
	let location = connection.query_row("Select sValue from Preferences WHERE Key='Location'", [], |row| {
			row.get::<usize, GPSCoord>(0)
		})?;
	connection.execute("INSERT INTO Preferences (Key, fValue) VALUES ('Latitude', ?1)", [location.Latitude])?;
	connection.execute("INSERT INTO Preferences (Key, fValue) VALUES ('Longitude', ?1)", [location.Longitude])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='TempHome'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='TempAway'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='TempComfort'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='DegreeDaysBaseTemperature'", [])?;
	Ok(())
}

fn update_users(connection: &Connection) -> Result<(), Box<dyn Error>> {
	connection.execute("ALTER TABLE Users ADD Salt VARCHAR(40) NOT NULL DEFAULT ''", [])?;
	connection.execute("UPDATE Users SET Password='', Active=FALSE", [])?;
	let mut stmt = connection.prepare("SELECT ID, Username FROM Users")?;
	let id_users : Vec<Result<(usize, String), _>>= stmt.query_map([], |row| {
		let id=row.get::<usize, usize>(0)?;
		let username = row.get::<usize, String>(1)?;
		Ok((id, username))
	})?.collect();
	for (id, username) in id_users.into_iter().flatten() {
		let dec = base64::decode(username)?;
		let str=dec.as_slice();
		let usernamedec=std::str::from_utf8(str)?.to_string();
		let salt=format!("{:X}",Sha256::digest(Utc::now().naive_local().to_string()+usernamedec.as_str()));
		connection.execute("UPDATE Users SET Username=?1, Salt=?3 WHERE ID=?2", (usernamedec,id,salt))?;
	}
	connection.execute("DELETE FROM UserSessions", [])?;
	Ok(())
}

fn drop_useless_tables(connection: &Connection) -> Result<(), Box<dyn Error>> {
	connection.execute("DROP TABLE EnOceanNodes", [])?;
	connection.execute("DROP TABLE LightSubDevices", [])?;
	connection.execute("DROP TABLE MySensors", [])?;
	connection.execute("DROP TABLE MySensorsChilds", [])?;
	connection.execute("DROP TABLE MySensorsVars", [])?;
	connection.execute("DROP TABLE PushLink", [])?;
	connection.execute("DROP TABLE SharedDevices", [])?;
	connection.execute("DROP TABLE ToonDevices", [])?;
	connection.execute("DROP TABLE ZWaveNodes", [])?;
	Ok(())
}

fn copy_and_upgrade_domoticz_db() -> Result<(), Box<dyn Error>> {
	std::fs::copy("Domoticz.db", "domorust.db")?;
	let connection=Connection::open("domorust.db")?;
	let dbversion = crate::db::settings::get_setting_int("DB_Version")?;
	Ok(#[allow(clippy::comparison_chain)]
	if dbversion > DB_VERSION as i64 {
		//User is using a newer database on a old Domoticz version
		//This is very dangerous and should not be allowed
		panic!("This Domoticz database incompatible with this Domorust version. An implementation is needed in Domorust to handle this...")
	} else if dbversion < DB_VERSION as i64 {
		//TODO: Do domoticz migration
		if dbversion < 165 {
			eprintln!(r#"
			Domoticz database is in a anterior version than the ones handled by domorust.
			Domorust will now try to upgrade your dababase but if something got wrong:
			You need to upgrade to a more recent domoticz format (by running a more recent domoticz with this database).
			
			Your version {}, min handled {}
		"#, dbversion, 165)
		}
		if dbversion < 166 {
			connection.execute("ALTER TABLE Notifications ADD COLUMN [Active] BOOLEAN DEFAULT true", [])?;
		}
		if dbversion < 167 {
			connection.execute("ALTER TABLE DeviceStatus ADD COLUMN [OrgHardwareID] INTEGER default 0", [])?;
		}
		if dbversion < 168 {
			connection.execute("ALTER TABLE Meter ADD COLUMN [Price] FLOAT DEFAULT 0", [])?;
			connection.execute("ALTER TABLE Meter_Calendar ADD COLUMN [Price] FLOAT DEFAULT 0", [])?;
			connection.execute("ALTER TABLE MultiMeter ADD COLUMN [Price] FLOAT DEFAULT 0", [])?;
			connection.execute("ALTER TABLE MultiMeter_Calendar ADD COLUMN [Price] FLOAT DEFAULT 0", [])?;
		}
		//if (dbversion < 169)
		// new version of BleBox module
		// BleBox not handled by domorust and will never be: ignore migration
	})
}

fn ensure_constraints(connection:&Connection, table:&str, createString:&str) -> Result<(), Box<dyn Error>>{
	connection.execute("PRAGMA foreign_keys=off;", [])?;
	connection.execute("BEGIN TRANSACTION;", [])?;
	connection.execute(("ALTER TABLE ".to_string() + table + " RENAME TO "+table+"_old;").as_str(), [])?;
	connection.execute(createString, [])?;
	connection.execute(("INSERT INTO ".to_string() + table + " SELECT * FROM " + table + "_old;").as_str(), [])?;
	connection.execute("COMMIT;", [])?;
	connection.execute("PRAGMA foreign_keys=on;", [])?;
	connection.execute(("DROP TABLE ".to_string()+table+"_old").as_str(), [])?;
	Ok(())
}
