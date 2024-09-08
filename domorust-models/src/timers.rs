#![allow(non_snake_case)]

use std::{error::Error, fmt::Display, str::FromStr};

use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Utc};
use domorust_macros::{FromHashMap, FromSqlRow, FromSqlTable, ToSqlQuery};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use serde::Serialize;
use serde_repr::Serialize_repr;

use crate::{FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery};

#[derive(Clone, Debug, Default, Serialize_repr, Copy)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum TimerType {

	BeforeSunrise = 0,
	AfterSunrise,
	#[default]
	OnTime,
	BeforeSunset,
	AfterSunset,
	FixedDateTime,
	DaysOdd,
	DaysEven,
	WeeksOdd,
	WeeksEven,
	Monthly,
	MonthlyWeekday,
	Yearly,
	YearlyWeekday,
	BeforeSunAtSouth,
	AfterSunAtSouth,
	BeforeCivilTwighlightStart,
	AfterCivilTwighlightStart,
	BeforeCivilTwighlightEnd,
	AfterCivilTwighlightEnd,
	BeforeNauticalTwighlightStart,
	AfterNauticalTwighlightStart,
	BeforeNauticalTwighlightEnd,
	AfterNauticalTwighlightEnd,
	BeforeAstronomicalTwighlightStart,
	AfterAstronomicalTwighlightStart,
	BeforeAstronomicalTwighlightEnd,
	AfterAstronomicalTwighlightEnd,
}
impl FromStr for TimerType {
	type Err = ParseEnumError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		TimerType::from_u8(s.parse().map_err(|_| ParseEnumError{})?).ok_or(ParseEnumError{})
	}
}
impl FromSql for TimerType {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val=u8::try_from(value.as_i64()?).map_err(|e|{FromSqlError::Other(Box::new(e))})?;
		TimerType::from_u8(val)
			.ok_or(FromSqlError::Other(Box::new(ParseEnumError{})))
	}
}
impl ToSql for TimerType {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok((*self as u8).into())
	}
}
#[derive(Debug)]
pub struct ParseEnumError{}
impl std::error::Error for ParseEnumError{}
impl Display for ParseEnumError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Enable to parse TimerType (value is incorrect)")
	}
}

#[derive(Clone, Debug, Default, Serialize, FromSqlRow, FromSqlTable, FromHashMap, ToSqlQuery)]
#[table_name("Timers")]
pub struct Timer {
	#[serde(rename="idx", with="crate::utils::string")]
	#[primary_key]
	pub ID:usize,
	#[param_name("active")]
	pub Active:bool,
	#[param_name("command")]
	pub Cmd:u8,
	pub Color:String,
	#[param_name("date")]
	pub Date: String,
	#[param_name("days")]
	pub Days:u16,
	#[param_name("vunit")]
	pub DeviceRowID: usize,
	#[param_name("level")]
	pub Level:u8,
	pub MDay:u8,
	pub Month:u8,
	pub Occurence:u8,
	//#[param_name("command")]
	//pub Persistant:bool,
	#[param_name("randomness")]
	#[column_name("UseRandomness")]
	pub Randomness:bool,
	#[param_name("timertype")]
	pub Time: String,
	#[serde(skip)]
	pub TimerPlan: usize,
	#[param_name("timertype")]
	pub Type: TimerType,
}
#[derive(Clone, Debug, Default, Serialize, FromSqlRow, FromSqlTable, ToSqlQuery, FromHashMap)]
#[table_name("TimerPlans")]
pub struct TimerPlan {
	#[serde(rename="idx")]
	#[primary_key]
	pub ID:usize,
	#[skip_field]
	pub Active:bool,
	pub Name:String,
}
pub fn sun_rise_set(latitude:f64, longitude:f64, now: DateTime<Local>) -> (DateTime<Local>, DateTime<Local>) {
	let (sunrise_unix, sunset_unix) = sunrise::sunrise_sunset(latitude, longitude, now.year(), now.month(), now.day());
	(
		DateTime::from_timestamp(sunrise_unix, 0).unwrap_or(Utc::now()).into(),
		DateTime::from_timestamp(sunset_unix, 0).unwrap_or(Utc::now()).into()
	)
}
pub fn get_next_time_of_timer(timer:&Timer, latitude: f64, longitude : f64) -> Result<DateTime<Local>, Box<dyn Error>> {
	let now = Local::now();
	match timer.Type {
		TimerType::OnTime => {
			let time = NaiveTime::parse_from_str(timer.Time.as_str(), "%H:%M")?;
			get_next_instant_from_time(time, false, now)
		},
		TimerType::AfterSunrise => {
			let time = NaiveTime::parse_from_str(timer.Time.as_str(), "%H:%M")?;
			let (rise, _set)=sun_rise_set(latitude, longitude, now);
			get_next_instant_from_time(time, false, rise)
		},
		TimerType::BeforeSunrise => {
			let time = NaiveTime::parse_from_str(timer.Time.as_str(), "%H:%M")?;
			let (rise, _set)=sun_rise_set(latitude, longitude, now);
			get_next_instant_from_time(time, true, rise)
		},
		TimerType::AfterSunset => {
			let time = NaiveTime::parse_from_str(timer.Time.as_str(), "%H:%M")?;
			let (_rise, set)=sun_rise_set(latitude, longitude, now);
			get_next_instant_from_time(time, false, set)
		},
		TimerType::BeforeSunset => {
			let time = NaiveTime::parse_from_str(timer.Time.as_str(), "%H:%M")?;
			let (_rise, set)=sun_rise_set(latitude, longitude, now);
			get_next_instant_from_time(time, true, set)
		},
		/*TimerType::DaysEven => {
		},
		TimerType::DaysOdd => {
		},
		TimerType::FixedDateTime => {
		},
		TimerType::Monthly => {
		},
		TimerType::MonthlyWeekday => {
		},
		TimerType::Yearly => {
		},
		TimerType::YearlyWeekday => {
		},*/
		_ => {
			Err("Not Implemented".into())
		}
	}
}

fn get_next_instant_from_time(time: NaiveTime, before: bool,now: DateTime<Local>) -> Result<DateTime<Local>, Box<dyn Error>> {
	let time_delta = time - now.time();
	let mut seconds = time_delta.num_seconds();
	if before {
		seconds = -seconds;
	}
	if seconds > 0 {
		Ok(now + time_delta)
	} else {
		Ok(now + time_delta + Duration::days(1))
	}
}
/*#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_get_next_instant_from_time() {
		let now = Local::now();
		assert!(get_next_instant_from_time(NaiveTime::parse_from_str("23:56:04", "%H:%M:%S"), false, now) < 0);
	}
}
*/