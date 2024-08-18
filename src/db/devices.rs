use core::f32;
use std::collections::HashMap;
use std::error::Error;

use domorust_models::custom_images;
use domorust_models::device::DeviceData;
use domorust_models::FromSqlTable;
use rusqlite::Connection;

use crate::domoticz::consts::{pTypeBARO, pTypeHUM, pTypeTEMP, pTypeTEMP_BARO, pTypeTEMP_HUM, DEVICE_SUBTYPES_DES, DEVICE_TYPES_DESC, SWITCH_TYPES};
use crate::domoticz::{get_humidity_status, is_light_or_switch, is_light_switch, is_temp, is_utility, is_weather};

pub fn get_devices(params: HashMap<String,String>) -> Result<Vec<DeviceData>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=DeviceData::build_from_table(&connection, &params);
	if let Ok(mut devices) = res {
		for d in &mut devices {
			d.SwitchTypeName = SWITCH_TYPES[d.SwitchType].to_string();
			let type_desc=DEVICE_TYPES_DESC.get(&(d.Type as u8)).unwrap_or(&("",""));
			d.TypeName = type_desc.0.to_string();
			d.TypeImg = type_desc.1.to_string();
			d.SubTypeName = DEVICE_SUBTYPES_DES.get(&(d.Type as u8, d.SubType as u8)).unwrap_or(&"").to_string();
			//d.LevelNames=async_session::base64::encode("Off|On").to_string();
			d.Temperature = f32::NAN;
			d.Humidity = f32::NAN;
			d.Barometer = u8::MAX;
			if d.CustomImage > 0 {
				if let Some(img)=custom_images::get_image(d.CustomImage) {
					d.Image=img.imageSrc.clone();
				}
			}
			if d.Type as u8 == pTypeTEMP {
				d.Temperature = d.Data.parse().unwrap_or(f32::NAN);
			}
			else if d.Type as u8 == pTypeTEMP_HUM {
				let vals : Vec<&str> = d.Data.split(';').collect();
				if vals.len() >= 2 {
					d.Temperature = vals[0].parse().unwrap_or(f32::NAN);
					d.Humidity = vals[1].parse().unwrap_or(f32::NAN);
				}
			}
			else if d.Type as u8 == pTypeTEMP_BARO {
				let vals : Vec<&str> = d.Data.split(';').collect();
				if vals.len() >= 2 {
					d.Temperature = vals[0].parse().unwrap_or(f32::NAN);
					d.Barometer = vals[1].parse().unwrap_or(u8::MAX);
				}
			}
			else if d.Type as u8 == pTypeHUM {
				d.Humidity = d.Data.parse().unwrap_or(f32::NAN);
				d.HumidityStatus = get_humidity_status(d.Humidity).to_string();
			}
			else if d.Type as u8 == pTypeBARO {
				d.Barometer = d.Data.parse().unwrap_or(u8::MAX);
			}
		}
		if params.contains_key("filter") {
			let filter = params.get("filter").unwrap();
			devices.retain(|d| {
				if filter == "temp" {
					return is_temp(d.Type as u8, d.SubType as u8);
				}
				else if filter == "weather" {
					return is_weather(d.Type as u8, d.SubType as u8);
				}
				else if filter == "light" {
					return is_light_or_switch(d.Type as u8, d.SubType as u8);
				}
				else if filter == "utility" {
					return is_utility(d.Type as u8, d.SubType as u8);
				}
				true
			});
		}
		return Ok(devices)
	}
	Ok(res?)
}

pub fn get_device(id: usize, params: HashMap<String,String>) -> Result<DeviceData, Box<dyn Error>> {
	let mut params = params.clone();
	params.insert("idx".to_string(), id.to_string());
	let mut dev =  get_devices(params)?;
	// TODO: if size > 1
	Ok(dev.pop().ok_or("Not found")?)
}
pub fn get_hardware_devices(hid: usize, params: HashMap<String,String>) -> Result<Vec<DeviceData>, Box<dyn Error>> {
	let mut params = params.clone();
	params.insert("HardwareID".to_string(), hid.to_string());
	get_devices(params)
}
pub fn get_light_switches(params: HashMap<String,String>)  -> Result<Vec<DeviceData>, Box<dyn Error>> {
	match get_devices(params) {
		Ok(mut devices) => {
			devices.retain(|d| {is_light_switch(d.Type as u8)});
			Ok(devices)
		},
		Err(e) => {
			Err(e)
		}
	}
	
}
pub fn make_favorite_device(id:usize, favorite:bool) -> Result<(), Box<dyn Error>>  {
	let connection = Connection::open("domorust.db")?;
	let writen = connection.execute("UPDATE DeviceStatus SET Favorite=?1 WHERE ID=?2", (favorite, id))?;
	if writen != 1 {
		Err("Request does not updated exactly one row".into())
	} else {
		Ok(())
	}
}