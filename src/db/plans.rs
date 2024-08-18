use std::{collections::HashMap, error::Error};
use domorust_models::device::DeviceData;
use domorust_models::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use domorust_models::plans::{FloorPlan, Plan};
use rusqlite::Connection;


pub fn get_floorplans(params: HashMap<String, String>) -> Result<Vec<FloorPlan>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut res=FloorPlan::build_from_table(&connection, &params)?;
	for d in res.iter_mut() {
		d.Image="domorust-api/floorplans/".to_string()+d.ID.to_string().as_str()+"/image";
		d.Plans = connection.query_row("SELECT COUNT(*) FROM Plans WHERE FloorplanID=?1", [d.ID], |row| {
			row.get::<usize, usize>(0)
		})?;
	} 
	Ok(res)
}
pub fn get_floorplan(idx: usize,params: HashMap<String, String>) -> Result<FloorPlan, Box<dyn Error>> {
	let mut params=params.clone();
	params.insert("idx".to_string(), idx.to_string());
	get_floorplans(params)?.pop().ok_or(rusqlite::Error::QueryReturnedNoRows.into())
}
pub fn get_floorplan_image(idx: usize) -> Result<Vec<u8>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut stmt = connection.prepare("SELECT Image FROM FloorPlans WHERE ID=?1")?;
	let res=stmt.query_row([idx], |row| {
		row.get::<usize,Vec<u8>>(0)
	});
	Ok(res?)
}
pub fn get_plans(params: HashMap<String, String>) -> Result<Vec<Plan>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let res=Plan::build_from_table(&connection, &params)?;
	Ok(res)
}
pub fn get_plan(idx:usize, params: HashMap<String, String>) -> Result<Plan, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut params=params.clone();
	params.insert("idx".to_string(), idx.to_string());
	let mut res=Plan::build_from_table(&connection, &params)?;
	// TODO: Raise error if more than one row
	res.pop().ok_or(rusqlite::Error::QueryReturnedNoRows.into())
}
pub fn add_plan(floorplanid:usize, params: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut var=Plan::from_hashmap(&params)?;
	var.FloorPlanID = floorplanid;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_plan(idx:usize, params: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut var=Plan::from_hashmap(&params)?;
	var.FloorPlanID = idx;
	var.update_query(&connection)?;
	Ok(())
}
pub fn delete_plan(idx:usize) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let nb = connection.execute("DELETE FROM Plan WHERE ID==?1", (idx,))?;
	if nb == 0 {
		return Err(rusqlite::Error::QueryReturnedNoRows.into());
	}
	else if nb > 1 {
		return Err("More than 1 plan has been deleted".into());
	}
	Ok(())
}

pub fn get_plan_devices(floorplanidx:usize, _params: HashMap<String, String>) -> Result<Vec<DeviceData>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "SELECT * FROM DeviceStatus WHERE ID IN (SELECT DeviceRowID FROM DeviceToPlansMap WHERE PlanID = ?1)";
	let mut stmt = connection.prepare(query)?;
	let mut res = vec![];
	let dev_iter=stmt.query_map([floorplanidx], |row| {
		DeviceData::build_from_row(row)
	})?;
	for h in dev_iter.flatten() {
		res.push(h);
	}
	Ok(res)
}