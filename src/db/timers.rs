use std::{collections::HashMap, error::Error};

use rusqlite::Connection;

use domorust_models::timers::{Timer, TimerPlan};
use domorust_models::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};


pub fn get_device_timers(dev_idx:usize) -> Result<Vec<Timer>, Box<dyn Error>> {
	let mut res=vec![];
	let connection = Connection::open("domorust.db").unwrap();
	let query = "SELECT * FROM Timers WHERE DeviceRowID=?1";
	let mut stmt = connection.prepare(query)?;
	
	let timers_iter = stmt.query_map([dev_idx], |row| {
		let timer = Timer::get_from_row(row)?;
		Ok(timer)
	})?;
	for t in timers_iter {
		res.push(t?)
	}
	Ok(res)
}
pub fn get_timers(filters: HashMap<String, String>) -> Result<Vec<Timer>, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	Timer::get_items_from_table(&connection, &filters)
}

pub fn get_timer(id: usize) -> Result<Timer, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	Timer::get_item_from_table(&connection, id)
}
pub fn add_timer(params:&HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let timer = Timer::from_hashmap(params)?;
	timer.add_query(&connection)?;
	Ok(())
}
pub fn update_timer(idx:usize, params:&HashMap<String,String>) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let mut timer = Timer::get_item_from_table(&connection, idx)?;
	timer.update_from_hashmap(&params)?;
	timer.update_query(&connection)?;
	Ok(())
}
pub fn delete_timer(idx:usize) -> Result<(), rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let query = "DELETE FROM Timers WHERE ID=?1";
	connection.execute(query,[idx])?;
	Ok(())
}
pub fn delete_device_timers(dev_id:usize) -> Result<(), rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let query = "DELETE FROM Timers WHERE DeviceRowID=?1";
	connection.execute(query,[dev_id])?;
	Ok(())
}

pub fn get_timerplans(filters: HashMap<String, String>) -> Result<Vec<TimerPlan>, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let mut timer_plans = TimerPlan::get_items_from_table(&connection, &filters)?;
	let query = "SELECT nValue FROM Preferences WHERE Key='ActiveTimerPlan'";
	let mut stmt = connection.prepare(query)?;
	let mut active_plan = stmt.query([])?;
	if let Ok(Some(row)) = active_plan.next() {
		let idx = row.get::<usize, usize>(0)?;
		let _=timer_plans.iter_mut().map(|tp| {tp.Active = idx == tp.ID});
	}
	Ok(timer_plans)
}
pub fn get_timerplan(id:usize) -> Result<TimerPlan, rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let mut tp = TimerPlan::get_item_from_table(&connection, id)?;
	let query = "SELECT nValue FROM Preferences WHERE Key='ActiveTimerPlan'";
	let mut stmt = connection.prepare(query)?;
	let mut active_plan = stmt.query([])?;
	if let Ok(Some(row)) = active_plan.next() {
		let idx = row.get::<usize, usize>(0)?;
		let _=tp.Active = idx == tp.ID;
	}
	Ok(tp)
}
pub fn add_timerplan(name: &String) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "INSERT INTO TimerPlans (Name) VALUES(?1)";
	connection.execute(query,(name,))?;
	Ok(())
}
pub fn update_timerplan(id:usize, name: &String) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "UPDATE TimerPlans SET Name=?2 WHERE ID=?1";
	connection.execute(query,(id, name))?;
	Ok(())
}
pub fn delete_timerplan(id:usize) -> Result<(), rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let query = "DELETE FROM TimerPlans WHERE ID=?1";
	connection.execute(query,[id])?;
	Ok(())
}
pub fn duplicate_timerplan(id:usize, name: &String) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	// TODO: transaction
	let query = "INSERT INTO TimerPlans (Name) VALUES(?1)";
	let _nb = connection.execute(query,(name,))?;
	let new_id= connection.last_insert_rowid();
	let tp = get_timerplan(id)?;
	let mut filters = HashMap::new();
	filters.insert("k".to_string(), tp.ID.to_string());
	let timers = get_timers(filters)?;
	for t in timers {
		let mut t_copy=t.clone();
		t_copy.TimerPlan = new_id as usize;
		t_copy.add_query(&connection)?;
	}
	//TODO: select timers of this timerplan and duplicate them
	Ok(())
}