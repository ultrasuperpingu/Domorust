use std::{collections::HashMap, error::Error};

use num_traits::FromPrimitive;
use rusqlite::Connection;

use domorust_models::timers::{Timer, TimerPlan, TimerType};
use domorust_models::FromSqlTable;


pub fn get_device_timers(dev_idx:usize) -> Result<Vec<Timer>, Box<dyn Error>> {
	let mut res=vec![];
	let connection = Connection::open("domorust.db").unwrap();
	let query = "SELECT * FROM Timers WHERE DeviceRowID=?1";
	let mut stmt = connection.prepare(query)?;
	
	let timers_iter = stmt.query_map([dev_idx], |row| {
		let timer=Timer {
			ID:row.get::<usize,usize>(0)?,
			Active:row.get(1)?,
			Cmd:row.get(6)?,
			Color:row.get(8)?,
			Date: row.get(3)?,
			Days:row.get(11)?,
			DeviceRowID:row.get(2)?,
			Level:row.get(7)?,
			MDay:row.get(13)?,
			Month:row.get(12)?,
			Occurence:row.get(14)?,
			Persistant:false,
			Randomness:row.get(9)?,
			Time: row.get(4)?,
			Type: FromPrimitive::from_u8(row.get::<usize, u8>(5)?).unwrap_or(TimerType::OnTime),
		};
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
pub fn add_timer(dev_id:usize, timer:&Timer) -> Result<(), rusqlite::Error> {
	let connection = Connection::open("domorust.db")?;
	let query = "INSERT INTO Timers (Active,DeviceRowID,Date,Time,Type,Cmd,Level,Color,UseRandomness,Days,Month,MDay,Occurence) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)";
	connection.execute(query, (timer.Active, dev_id,&timer.Date, &timer.Time, timer.Type as u8, timer.Cmd, timer.Level, &timer.Color, &timer.Color, timer.Randomness, timer.Days, timer.Month, timer.MDay, timer.Occurence))?;
	Ok(())
}
pub fn update_timer(idx:usize, timer:&Timer) -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db")?;
	let query = "UPDATE Timers SET Active=?1,Date=?2,Time=?3,Type=?4,Cmd=?5,Level=?6,Color=?7,UseRandomness=?8,Days=?9,Month=?10,MDay=?11,Occurence=?12 WHERE ID == ?13";
	connection.execute(query, (timer.Active, &timer.Date, &timer.Time, timer.Type as u8, timer.Cmd, timer.Level, &timer.Color, &timer.Color, timer.Randomness, timer.Days, timer.Month, timer.MDay, timer.Occurence, idx))?;
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
		let _=timer_plans.iter_mut().map(|tp| {tp.Active = idx == tp.idx});
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
		let _=tp.Active = idx == tp.idx;
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
	let query = "INSERT INTO TimerPlans (Name) VALUES(?1)";
	let _nb = connection.execute(query,(name,))?;
	let _t = get_timerplan(id)?;
	//TODO: select timers of this timerplan and duplicate them
	Ok(())
}