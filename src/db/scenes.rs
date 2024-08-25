use std::{collections::HashMap, error::Error};

use domorust_models::{scenes::{Scene, SceneDevice}, timers::Timer};
use domorust_models::{FromHashMap, FromSqlTable, ToSqlQuery};
use rusqlite::Connection;

pub fn get_scenes(params: HashMap<String, String>) -> Result<Vec<Scene>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=Scene::get_items_from_table(&connection, &params)?;
	Ok(res)
}
pub fn get_scene(id: usize) -> Result<Scene, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=Scene::get_item_from_table(&connection, id)?;
	Ok(res)
}
pub fn add_scene(params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	let var=Scene::from_hashmap(&params)?;
	println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}

pub fn update_scene(id:usize, params:HashMap<String,String>) -> Result<(), Box<dyn Error>> {

	let connection = Connection::open("domorust.db")?;
	//TODO: do not read in db to merge, just make the update on provided fields
	let mut p=get_scene(id)?;
	p.update_from_hashmap(&params)?;
	p.update_query(&connection)?;
	Ok(())
}

pub fn delete_scene(idx: usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	let query = "DELETE FROM Scenes WHERE ID==?1";
	let _ = connection.execute(query, (idx,))?;
	// TODO: Delete Scene Devices/Timers/Activations...
	Ok(())
}

pub fn get_scenes_devices(idx:usize, params:HashMap<String,String>) -> Result<Vec<SceneDevice>, Box<dyn Error>>  {
	let connection = Connection::open("domorust.db")?;
	let mut params = params.clone();
	params.insert("idx".to_string(), idx.to_string());
	let res=SceneDevice::get_items_from_table(&connection, &params)?;
	Ok(res)
}

pub fn get_scenes_activations(_idx:usize, _params: HashMap<String, String>) -> Result<Vec<String>, Box<dyn Error>>  {
	Err("Not Implemented".into())
}

pub fn get_scene_timers(_idx:usize, _params: HashMap<String, String>) -> Result<Vec<Timer>, Box<dyn Error>>  {
	Err("Not Implemented".into())
}

pub fn get_scene_logs(_idx:usize, _params: HashMap<String, String>) -> Result<Vec<String>, Box<dyn Error>>  {
	Err("Not Implemented".into())
}

pub fn add_scene_device(_idx:usize, _params: HashMap<String, String>) -> Result<(), Box<dyn Error>>  {
	Err("Not Implemented".into())
}

pub fn add_scene_timer(_idx:usize, _params: HashMap<String, String>) -> Result<(), Box<dyn Error>>  {
	Err("Not Implemented".into())
}

pub fn add_scene_activation(_idx:usize, _params: HashMap<String, String>) -> Result<(), Box<dyn Error>>  {
	Err("Not Implemented".into())
}
