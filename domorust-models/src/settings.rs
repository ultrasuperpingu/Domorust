#![allow(non_snake_case)]

use std::{collections::HashMap, fmt::Display, str::FromStr};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use domorust_macros::{FromSqlRowFields, ToSqlRowFields};
use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use serde::Serialize;
use serde_repr::Serialize_repr;


use crate::{timers::ParseEnumError, FromSqlRowFields, ParseDomorustTypeError, ToSqlRowFields};

#[derive(Clone, Debug, Default, Serialize)]
#[derive(FromSqlRowFields, ToSqlRowFields)]
#[table_name("Preferences")]
#[name_column_name("Key")]
#[name_column_idx(0)]
#[value_column_name("sValue")]
#[value_column_idx(2)]
pub struct Settings
{
	#[value_column_name("nValue")]
	pub AcceptNewHardware:u8, // : 0,
	#[value_column_name("nValue")]
	pub ActiveTimerPlan:u8, // : 1,
	#[value_column_name("nValue")]
	pub AllowPlainBasicAuth:u8, // : 0,
	#[value_column_name("nValue")]
	pub AllowWidgetOrdering:u8, // : 1,
	#[value_column_name("nValue")]
	//#[serde(rename="BatterLowLevel")]
	pub BatteryLowNotification:u8, // : 0,
	#[value_column_name("nValue")]
	pub CM113DisplayType:u8, // : 0,
	#[serde(with="crate::utils::base64_encoded")]
	pub ClickatellAPI:String, // : "",
	#[value_column_name("nValue")]
	pub ClickatellEnabled:u8, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub ClickatellFrom:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub ClickatellTo:String, // : "",
	pub CostEnergy:String, // : "0.2682",
	pub CostEnergyR1:String, // : "0.0800",
	pub CostEnergyR2:String, // : "0.0800",
	pub CostEnergyT2:String, // : "0.2049",
	pub CostGas:String, // : "0.6218",
	pub CostWater:String, // : "1.6473",
	pub DashboardType:u8, // : 1,
	pub DegreeDaysBaseTemperature:String, // : "18.0",
	#[value_column_name("nValue")]
	pub DisableDzVentsSystem:u8, // : 0,
	#[value_column_name("nValue")]
	pub DoorbellCommand:u8, // : 0,
	#[value_column_name("nValue")]
	pub DzVentsLogLevel:u8, // : 2,
	#[value_column_name("nValue")]
	pub ElectricVoltage:u16, // : 230,
	pub EmailAsAttachment:String, // : "",
	#[value_column_name("nValue")]
	pub EmailEnabled:u8, // : 1,
	pub EmailFrom:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub EmailPassword:String, // : "",
	#[value_column_name("nValue")]
	pub EmailPort:u16, // : 25,
	pub EmailServer:String, // : "",
	pub EmailTo:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub EmailUsername:String, // : "",
	pub EnableEventScriptSystem:u8, // : 1,
	#[value_column_name("nValue")]
	//#[serde(rename="EnergyDivider")]
	pub MeterDividerEnergy:u32, // : 1000,
	pub EventSystemLogFullURL:u8, // : 0,
	pub FCMEnabled:String, // : "",
	#[value_column_name("nValue")]
	pub FloorplanActiveOpacity:u8, // : 25,
	#[value_column_name("nValue")]
	pub FloorplanAnimateZoom:u8, // : 1,
	#[value_column_name("nValue")]
	pub FloorplanFullscreenMode:u8, // : 0,
	#[value_column_name("nValue")]
	pub FloorplanInactiveOpacity:u8, // : 5,
	#[value_column_name("nValue")]
	pub FloorplanPopupDelay:u32, // : 750,
	pub FloorplanRoomColour:String, // : "Blue",
	#[value_column_name("nValue")]
	pub FloorplanShowSceneNames:u8, // : 0,
	#[value_column_name("nValue")]
	pub FloorplanShowSensorValues:u8, // : 1,
	#[value_column_name("nValue")]
	pub FloorplanShowSwitchValues:u8, // : 0,
	//#[serde(rename="GasDivider")]
	pub MeterDividerGas:u32, // : 100,
	pub HTTPEnabled:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPField1:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPField2:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPField3:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPField4:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPPostContentType:String, // : "YXBwbGljYXRpb24vanNvbg==",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPPostData:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPPostHeaders:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPTo:String, // : "",
	#[serde(with="crate::utils::base64_encoded")]
	pub HTTPURL:String, // : "aHR0cHM6Ly93d3cuc29tZWdhdGV3YXkuY29tL3B1c2h1cmwucGhwP3VzZXJuYW1lPSNGSUVMRDEmcGFzc3dvcmQ9I0ZJRUxEMiZhcGlrZXk9I0ZJRUxEMyZmcm9tPSNGSUVMRDQmdG89I1RPJm1lc3NhZ2U9I01FU1NBR0U=",
	pub HideDisabledHardwareSensors:u8, // : 1,
	#[serde(with="crate::utils::base64_encoded")]
	pub IFTTTAPI:String, // : "",
	#[value_column_name("nValue")]
	pub IFTTTEnabled:u8, // : 0,
	pub KodiEnabled:String, // : "",
	pub KodiIPAddress:String, // : "224.0.0.1",
	#[value_column_name("nValue")]
	pub KodiPort:u16, // : 9777,
	#[value_column_name("nValue")]
	pub KodiTimeToLive:u16, // : 5,
	pub Language:String, // : "fr",
	#[value_column_name("nValue")]
	pub LightHistoryDays:u8, // : 7,
	#[value_column_name("nValue")]
	pub LmsDuration:u8, // : 5,
	pub LmsEnabled:String, // : "",
	pub LmsPlayerMac:String, // : "",
	pub Location : GPSCoord,
	#[value_column_name("nValue")]
	pub LogEventScriptTrigger:u8, // : 0,
	#[value_column_name("nValue")]
	pub MaxElectricPower:u32, // : 6000,
	#[value_column_name("nValue")]
	pub MobileType:u8, // : 0,
	#[value_column_name("nValue")]
	pub MyDomoticzSubsystems:u8, // : 0,
	#[value_column_name("nValue")]
	pub NotificationSensorInterval:u32, // : 43200,
	#[value_column_name("nValue")]
	pub NotificationSwitchInterval:u32, // : 0,
	pub ProtectionPassword:String, // : "d41d8cd98f00b204e9800998ecf8427e",
	pub ProwlAPI:String, // : "",
	pub ProwlEnabled:String, // : "",
	pub PushALotAPI:String, // : "",
	pub PushALotEnabled:String, // : "",
	pub PushbulletAPI:String, // : "",
	pub PushbulletEnabled:String, // : "",
	pub PushoverAPI:String, // : "",
	pub PushoverEnabled:String, // : "",
	pub PushoverUser:String, // : "",
	pub PushsaferAPI:String, // : "",
	pub PushsaferEnabled:String, // : "",
	pub PushsaferImage:String, // : "",
	#[value_column_name("nValue")]
	pub RandomTimerFrame:u32, // : 5,
	pub RaspCamParams:String, // : "-w 800 -h 600 -t 1",
	#[value_column_name("nValue")]
	pub ReleaseChannel:u32, // : 0,
	pub RemoteSharedPort:u16, // : 6144,
	#[value_column_name("nValue")]
	pub SecOnDelay:u32, // : 30,
	pub SecPassword:String, // : "9875d50b114e4c6f61e46de4b9a5131a",
	pub SendErrorsAsNotification:u8, // : 0,
	#[value_column_name("nValue")]
	pub SensorTimeout:u16, // : 90,
	pub ShortLogAddOnlyNewValues:u8, // : 0,
	#[value_column_name("nValue")]
	pub ShortLogDays:u16, // : 7,
	#[value_column_name("nValue")]
	pub ShortLogInterval:u16, // : 5,
	#[value_column_name("nValue")]
	pub ShowUpdateEffect:u8, // : 0,
	#[value_column_name("nValue")]
	pub SmartMeterType:u8, // : 0,
	pub TelegramAPI:String, // : "",
	pub TelegramChat:String, // : "",
	pub TelegramEnabled:String, // : "",
	#[value_column_name("nValue")]
	pub TempUnit:u8, // : 0,
	pub Title:String, // : "Domoticz",
	pub UVCParams:String, // : "-S80 -B128 -C128 -G80 -x800 -y600 -q100",
	#[value_column_name("nValue")]
	pub UseAutoBackup:u8, // : 0,
	#[value_column_name("nValue")]
	pub UseAutoUpdate:u8, // : 0,
	#[value_column_name("nValue")]
	pub UseEmailInNotifications:u8, // : 1,
	#[value_column_name("nValue")]
	//#[serde(rename="WaterDivider")]
	pub MeterDividerWater:u32, // : 100,
	pub WebLocalNetworks:String, // : "127.0.0.1;192.168.154.21;localhost;192.168.154.*",
	//#[param_name("Themes")]
	pub WebTheme:String, // : "default",
	#[value_column_name("nValue")]
	pub WeightUnit:u8, // : 0,
	#[value_column_name("nValue")]
	pub WindUnit:u8, // : 0,
	#[skip_field]
	pub cloudenabled:bool, // : false,
}

#[derive(Clone, Debug, Default, Serialize)]
#[derive(FromSqlRowFields)]
#[table_name("Preferences")]
#[name_column_name("Key")]
#[name_column_idx(0)]
#[value_column_name("sValue")]
#[value_column_idx(2)]
pub struct ConfigResponseSettings {
	#[value_column_name("nValue")]
	pub AllowWidgetOrdering : bool,
	#[value_column_name("nValue")]
	pub DashboardType : u8,
	#[serde(with="crate::utils::string_to_f32")]
	pub DegreeDaysBaseTemperature : f32,
	#[skip_field]
	pub DomoticzUpdateURL : String,
	#[name_value("5MinuteHistoryDays")]
	#[value_column_name("nValue")]
	pub FiveMinuteHistoryDays : u8,
	#[skip_field]
	pub HaveUpdate : bool,
	#[value_column_name("nValue")]
	pub MobileType : u8,
	#[skip_field]
	pub Revision : u16,
	#[skip_field]
	pub SystemName : String,
	#[value_column_name("nValue")]
	pub ShowUpdatedEffect : bool,
	#[value_column_name("nValue")]
	pub TempUnit : TempUnit,
	#[skip_field]
	pub TempScale : f32,
	#[skip_field]
	pub TempSign : String,
	#[serde(rename="UseUpdate")]
	#[value_column_name("nValue")]
	pub UseAutoUpdate : bool,
	#[skip_field]
	pub UserName : String,
	#[value_column_name("nValue")]
	pub WeightUnit : WindUnit,
	#[skip_field]
	pub WeightScale : f32,
	#[skip_field]
	pub WeightSign : String,
	#[value_column_name("nValue")]
	pub WindUnit : WindUnit,
	#[skip_field]
	pub WindScale : f32,
	#[skip_field]
	pub WindSign : String,
	#[skip_field]
	pub build_time : String,
	#[skip_field]
	pub dzvents_version : String,
	#[skip_field]
	pub hash : String,
	#[serde(rename="language")]
	pub Language : String,
	#[skip_field]
	pub python_version : String,
	//#[skip_field]
	// get in Users table
	pub result : TabsEnabled,
	#[serde(rename="version")]
	pub Domoticz_Version : String
}


#[derive(Debug, Default, Clone, Serialize)]
#[derive(FromSqlRowFields)]
#[name_column_name("Key")]
#[value_column_name("sValue")]
#[table_name("Preferences")]
pub struct FloorPlansConfig {
	#[value_column_name("nValue")]
	#[serde(rename="ActiveRoomOpacity")]
	pub FloorplanActiveOpacity:usize,//" : 25,
	#[value_column_name("nValue")]
	#[serde(rename="AnimateZoom")]
	pub FloorplanAnimateZoom:usize,//" : 1,
	#[value_column_name("nValue")]
	#[serde(rename="FullscreenMode")]
	pub FloorplanFullscreenMode:usize,//" : 1,
	#[value_column_name("nValue")]
	#[serde(rename="InactiveRoomOpacity")]
	pub FloorplanInactiveOpacity:usize,//" : 5,
	#[value_column_name("nValue")]
	#[serde(rename="PopupDelay")]
	pub FloorplanPopupDelay:usize,//" : 750,
	#[value_column_name("sValue")]
	#[serde(rename="RoomColour")]
	pub FloorplanRoomColour:String,//" : "Blue",
	#[value_column_name("nValue")]
	#[serde(rename="ShowSceneNames")]
	pub FloorplanShowSceneNames:usize,//" : 0,
	#[value_column_name("nValue")]
	#[serde(rename="ShowSensorValues")]
	pub FloorplanShowSensorValues:usize,//" : 1,
	#[value_column_name("nValue")]
	#[serde(rename="ShowSwitchValues")]
	pub FloorplanShowSwitchValues:usize,//" : 0,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct TabsEnabled {
	pub EnableTabCustom : bool,
	pub EnableTabDashboard : bool,
	pub EnableTabFloorplans : bool,
	pub EnableTabLights : bool,
	pub EnableTabScenes : bool,
	pub EnableTabTemp : bool,
	pub EnableTabUtility : bool,
	pub EnableTabWeather : bool
}

impl FromSql for TabsEnabled {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val = value.as_i64_or_null()?;
		if let Some(val) = val {
			let val = val as u8; 
			Ok(TabsEnabled {
				EnableTabCustom : val&0x20 != 0,
				EnableTabDashboard : val&0x1 != 0,
				EnableTabFloorplans : val&0x40 != 0,
				EnableTabLights : val&0x1 != 0,
				EnableTabScenes : val&0x2 != 0,
				EnableTabTemp : val&0x4 != 0,
				EnableTabUtility : val&0x10 != 0,
				EnableTabWeather : val&0x8 != 0
			})
		} else {
			Err(FromSqlError::InvalidType)
		}
	}
}
/*impl ToSql for TabsEnabled {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		
	}
}*/

#[derive(Copy, Clone, Debug, Default, Serialize_repr)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum WindUnit
{
	#[default]
	MS = 0,
	KMH,
	MPH,
	Knots,
	Beaufort,
}
impl WindUnit {
	pub fn value(&self) -> (&str, f32, f32) {
		match *self {
			WindUnit::MS => ("m/s", 1.0, 0.0),
			WindUnit::KMH => ("km/h", 3.6, 0.0),
			WindUnit::MPH => ("mph", 2.23693629205, 0.0),
			WindUnit::Knots => ("kn", 1.943844492457398, 0.0),
			WindUnit::Beaufort => ("bf", 10.0, 0.0),
		}
	}
}
impl FromSql for WindUnit {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val=u8::try_from(value.as_i64()?).map_err(|e|{FromSqlError::Other(Box::new(e))})?;
		WindUnit::from_u8(val)
			.ok_or(FromSqlError::Other(Box::new(ParseEnumError{})))
	}
}
impl ToSql for WindUnit {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok((*self as u8).into())
	}
}
#[derive(Copy, Clone, Debug, Default, Serialize_repr)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum TempUnit
{
	#[default]
	Celsius = 0,
	Fahrenheit,
}

impl TempUnit {
	pub fn value(&self) -> (&str, f32, f32) {
		match *self {
			TempUnit::Celsius => ("°C", 1.0, 0.0),
			TempUnit::Fahrenheit => ("°F", 1.8, 32.0), // 32+1.8
		}
	}
}
impl FromSql for TempUnit {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val=u8::try_from(value.as_i64()?).map_err(|e|{FromSqlError::Other(Box::new(e))})?;
		TempUnit::from_u8(val)
			.ok_or(FromSqlError::Other(Box::new(ParseEnumError{})))
	}
}
impl ToSql for TempUnit {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok((*self as u8).into())
	}
}
#[derive(Copy, Clone, Debug, Serialize_repr)]
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum WeightUnit
{
	Kg,
	Lb,
}
impl WeightUnit {
	pub fn value(&self) -> (&str, f32, f32) {
		match *self {
			WeightUnit::Kg => ("kg", 1.0, 0.0),
			WeightUnit::Lb => ("lb", 2.20462, 0.0),
		}
	}
}
impl FromSql for WeightUnit {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		let val=u8::try_from(value.as_i64()?).map_err(|e|{FromSqlError::Other(Box::new(e))})?;
		WeightUnit::from_u8(val)
			.ok_or(FromSqlError::Other(Box::new(ParseEnumError{})))
	}
}
impl ToSql for WeightUnit {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok((*self as u8).into())
	}
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GPSCoord
{
	pub Latitude: f64,// : "48.390528",
	pub Longitude: f64// : "-4.486009"
}
impl GPSCoord {
	pub fn to_tuple(&self) -> (f64, f64) {
		(self.Latitude,self.Longitude)
	}
}
impl Display for GPSCoord {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{};{}", self.Latitude, self.Longitude)
	}
}
impl FromSql for GPSCoord {
	fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
		value.as_str()?.parse()
			.map_err(|e| FromSqlError::Other(Box::new(e)))
	}
}
impl ToSql for GPSCoord {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok(self.to_string().into())
	}
}
impl FromStr for GPSCoord {
	type Err = ParseDomorustTypeError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(';')
			.ok_or(ParseDomorustTypeError::new("GPSCoord", s.to_string()))?;

		let x_fromstr = x.parse::<f64>().map_err(|_| ParseDomorustTypeError::new("GPSCoord", s.to_string()))?;
		let y_fromstr = y.parse::<f64>().map_err(|_| ParseDomorustTypeError::new("GPSCoord", s.to_string()))?;

		Ok(GPSCoord { Latitude: x_fromstr, Longitude: y_fromstr })
		//Ok(GPSCoord { Latitude: x.to_string(), Longitude: y.to_string() })
	}
}