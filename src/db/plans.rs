use std::{collections::HashMap, error::Error};
use domorust_models::device::Device;
use domorust_models::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use domorust_models::plans::{FloorPlan, Plan};
use rusqlite::Connection;


pub fn get_floorplans(params: HashMap<String, String>) -> Result<Vec<FloorPlan>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut res=FloorPlan::get_items_from_table(&connection, &params)?;
	for d in res.iter_mut() {
		d.Image="domorust-api/floorplans/".to_string()+d.ID.to_string().as_str()+"/image";
		d.Plans = connection.query_row("SELECT COUNT(*) FROM Plans WHERE FloorplanID=?1", [d.ID], |row| {
			row.get::<usize, usize>(0)
		})?;
	} 
	Ok(res)
}
pub fn get_floorplan(id: usize) -> Result<FloorPlan, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let mut res = FloorPlan::get_item_from_table(&connection, id)?;
	res.Image="domorust-api/floorplans/".to_string()+res.ID.to_string().as_str()+"/image";
	res.Plans = connection.query_row("SELECT COUNT(*) FROM Plans WHERE FloorplanID=?1", [res.ID], |row| {
		row.get::<usize, usize>(0)
	})?;
	Ok(res)
}
pub fn get_floorplan_image(id: usize) -> Result<Vec<u8>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut stmt = connection.prepare("SELECT Image FROM FloorPlans WHERE ID=?1")?;
	let res=stmt.query_row([id], |row| {
		row.get::<usize,Vec<u8>>(0)
	});
	Ok(res?)
}
pub fn get_plans(params: HashMap<String, String>) -> Result<Vec<Plan>, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	Plan::get_items_from_table(&connection, &params)
}
pub fn get_plan(id:usize) -> Result<Plan, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	Plan::get_item_from_table(&connection, id)
}
pub fn add_plan(floorplanid:usize, params: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut var=Plan::from_hashmap(&params)?;
	var.FloorPlanID = floorplanid;
	//println!("params:{:?} built:{:?}", params, var);
	var.add_query(&connection)?;
	Ok(())
}
pub fn update_plan(id:usize, params: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	//TODO: do not read in db to merge, just make the update on provided fields
	let mut p=get_plan(id)?;
	p.update_from_hashmap(&params)?;
	p.FloorPlanID = id;
	p.update_query(&connection)?;
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

pub fn get_plan_devices(floorplanidx:usize, _params: HashMap<String, String>) -> Result<Vec<Device>, Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "SELECT * FROM Devices WHERE ID IN (SELECT DeviceRowID FROM DeviceToPlansMap WHERE PlanID = ?1)";
	let mut stmt = connection.prepare(query)?;
	let mut res = vec![];
	let dev_iter=stmt.query_map([floorplanidx], |row| {
		Device::get_from_row(row)
	})?;
	for h in dev_iter.flatten() {
		res.push(h);
	}
	Ok(res)
}