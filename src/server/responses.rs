#![allow(non_snake_case)]


use std::collections::HashMap;
use std::fs;
use std::error::Error;
use chrono::Local;
use domorust_models::{domorust::DomorustConfig, timers::sun_rise_set};
use serde::Serialize;

use crate::{db, domoticz::consts::{METER_TYPES, SWITCH_TYPES}};
#[derive(Serialize, Debug, Clone)]
pub struct AuthResponse {
	rights : u16,
	status : &'static str,
	title : &'static str,
	user : String,
	version : &'static str
}
impl AuthResponse {
	pub fn unauthorized() -> Self {
		Self {
			rights: 255,
			status: "KO",
			title: "GetAuth",
			user: String::from(""),
			version: "2024.1"
		}
	}
	pub fn authorized(user:String, rights:u16) -> Self {
		Self {
			rights,
			status: "OK",
			title: "GetAuth",
			user,
			version: "2024.1"
		}
	}
}
impl Default for AuthResponse {
	fn default() -> Self {
		Self {
			rights: 2,
			status: "OK",
			title: "GetAuth",
			user: String::from("admin"),
			version: "2024.1"
		}
	}
}
#[derive(Serialize, Debug, Clone)]
pub struct ConfigResponse {
	AllowWidgetOrdering : bool,
	DashboardType : u8,
	DegreeDaysBaseTemperature : f32,
	DomoticzUpdateURL : &'static str,
	FiveMinuteHistoryDays : u8,
	HaveUpdate : bool,
	MobileType : u8,
	Revision : u16,
	SystemName : &'static str,
	TempScale : f32,
	TempSign : &'static str,
	UseUpdate : bool,
	UserName : String,
	WindScale : f32,
	WindSign : &'static str,
	build_time : &'static str,
	dzvents_version : &'static str,
	hash : &'static str,
	language : &'static str,
	python_version : &'static str,
	result : ResultConfigResponse,
	status : &'static str,
	title : &'static str,
	version : &'static str
}

impl Default for ConfigResponse {
	fn default() -> Self
	{
		Self {
			AllowWidgetOrdering : true,
			DashboardType : 1,
			DegreeDaysBaseTemperature : 18.0,
			DomoticzUpdateURL : "https://www.domoticz.com/download.php?channel=stable&type=release&system=linux&machine=armv7l",
			FiveMinuteHistoryDays : 7,
			HaveUpdate : false,
			MobileType : 0,
			Revision : 15889,
			SystemName : "linux",
			TempScale : 1.0,
			TempSign : "C",
			UseUpdate : true,
			UserName : String::from("admin"),
			WindScale : 1.0,
			WindSign : "m/s",
			build_time : "2023-02-14 15:06:40",
			dzvents_version : "3.1.8",
			hash : "f9b9ac774",
			language : "fr",
			python_version : "3.9.2 (default, Mar 12 2021, 04:06:34) \n[GCC 10.2.1 20210110]",
			result : ResultConfigResponse {
				EnableTabCustom : false,
				EnableTabDashboard : true,
				EnableTabFloorplans : false,
				EnableTabLights : true,
				EnableTabScenes : true,
				EnableTabTemp : true,
				EnableTabUtility : true,
				EnableTabWeather : true,
				ShowUpdatedEffect : false
			},
			status : "OK",
			title : "GetConfig",
			version : "2024.1"
		}
	}
}
#[derive(Serialize, Debug, Clone)]
struct ResultConfigResponse {
	EnableTabCustom : bool,
	EnableTabDashboard : bool,
	EnableTabFloorplans : bool,
	EnableTabLights : bool,
	EnableTabScenes : bool,
	EnableTabTemp : bool,
	EnableTabUtility : bool,
	EnableTabWeather : bool,
	ShowUpdatedEffect : bool
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RequestResult<T : Serialize>
{
	pub status : &'static str,
	title : &'static str,
	#[serde(skip_serializing_if="HashMap::is_empty", flatten)]
	pub additional_fields : HashMap<String, String>,
	#[serde(skip_serializing_if="Vec::is_empty")]
	pub result : Vec<T>,
}
impl<T : Serialize> RequestResult<T> {
	pub fn new(title:&'static str, result: Vec<T>) -> Self {
		Self{title,status:"OK",result, additional_fields:HashMap::new()}
	}
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RequestResult2<T : Serialize, U: Serialize>
{
	pub status : &'static str,
	title : &'static str,
	#[serde(flatten)]
	pub additional_fields : U,
	#[serde(skip_serializing_if="Vec::is_empty")]
	pub result : Vec<T>,
}
impl<T : Serialize, U: Serialize> RequestResult2<T, U> {
	pub fn new(title:&'static str, result: Vec<T>, additional_fields: U) -> Self {
		Self{title,status:"OK",result, additional_fields}
	}
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct RequestSingleResult<T : Serialize>
{
	pub status : &'static str,
	title : &'static str,
	#[serde(skip_serializing_if="HashMap::is_empty", flatten)]
	pub additional_fields : HashMap<String, String>,
	#[serde(flatten)]
	pub result : T,
}
impl<T : Serialize> RequestSingleResult<T> {
	pub fn new(title:&'static str, result: T) -> Self {
		Self{title,status:"OK",result, additional_fields:HashMap::new()}
	}
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct RequestError {
	pub error : String,
	status : &'static str,
	pub title : &'static str
}
impl RequestError {
	pub fn new(title: &'static str, e: Box<dyn Error>) -> Self {
		Self{title,status:"KO",error:e.to_string()}
	}
}




/*pub struct DeviceResponse
{
	ActTime : usize,// : 1722495400,
	AstrTwilightEnd : String,// : "00:17",
	AstrTwilightStart : String,// : "04:31",
	CivTwilightEnd : String,// : "22:31",
	CivTwilightStart : String,// : "06:17",
	DayLength : String,// : "15:01",
	NautTwilightEnd : String,// : "23:19",
	NautTwilightStart : String,// : "05:30",
	ServerTime : String,// : "2024-08-01 08:56:40",
	SunAtSouth : String,// : "14:24",
	Sunrise : String,// : "06:54",
	Sunset : String,// : "21:55",
	app_version : String,// : "2024.4",
	result : Vec<Device>,
	status : String,// "OK",
	title : String,// "Devices"
}*/

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSunRiseSetResponse {
	//pub AstrTwilightEnd : String,     //"01:56",
	//pub AstrTwilightStart : String,   //"02:43",
	//pub CivTwilightEnd : String,      //"23:04",
	//pub CivTwilightStart : String,    //"05:34",
	pub DayLength : String,           //"16:06",
	//pub NautTwilightEnd : String,     //"00:03",
	//pub NautTwilightStart : String,   //"04:35",
	pub ServerTime : String,          //"2024-06-18 22:48:23",
	//pub SunAtSouth : String,          //"14:19",
	pub Sunrise : String,             //"06:16",
	pub Sunset : String,              //"22:22",
	pub status : String,              //"OK",
	pub title : String                //"getSunRiseSet"
}

#[allow(clippy::field_reassign_with_default)]
pub fn get_sun_rise_set() -> Result<GetSunRiseSetResponse,Box<dyn Error>> {
	let mut sunconf=GetSunRiseSetResponse::default();
	sunconf.title=String::from("getSunRiseSet");
	sunconf.status=String::from("OK");
	let now=Local::now();
	sunconf.ServerTime = now.format("%Y-%m-%d %H:%M:%S").to_string();
	let (latitude,longitude) = db::get_latitude_longitude()?.to_tuple();
	let (sunrise_dt, sunset_dt) = sun_rise_set(latitude, longitude, now);
	sunconf.Sunrise = sunrise_dt.format("%H:%M").to_string();
	sunconf.Sunset = sunset_dt.format("%H:%M").to_string();
	let day_duration=sunset_dt.signed_duration_since(sunrise_dt);
	let day_dur_secs=day_duration.num_seconds();
	sunconf.DayLength=format!("{:02}:{:02}",(day_dur_secs / 60) / 60,(day_dur_secs / 60) % 60);
	Ok(sunconf)
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct NameValuePair<T> {
	pub name: String,
	value: T
}
pub fn get_serial_ports() -> Result<Vec<NameValuePair<usize>>, Box<dyn Error>> {
	let ports = serialport::available_ports()?;
	let mut list=vec![];
	for p in ports.iter().enumerate() {
		list.push(NameValuePair::<usize>{name:p.1.port_name.clone(),value:p.0});
	}
	Ok(list)
}


#[derive(Clone, Debug, Default, Serialize)]
pub struct GetLanguagesResponse {
	pub language:String,
	pub result:HashMap<String, String>,
	pub status: String,
	pub title: String
}
const GUI_LANGUAGE: [(&str, &str) ; 36] = [
	("en", "English"  ), ("sq", "Albanian"  ), ("ar", "Arabic"  ), ("bs", "Bosnian"     ), ("bg", "Bulgarian"), ("ca", "Catalan"  ),
	("zh", "Chinese"  ), ("cs", "Czech"     ), ("da", "Danish"  ), ("nl", "Dutch"       ), ("et", "Estonian" ), ("de", "German"   ),
	("el", "Greek"    ), ("fr", "French"    ), ("fi", "Finnish" ), ("he", "Hebrew"      ), ("hu", "Hungarian"), ("is", "Icelandic"),
	("it", "Italian"  ), ("lt", "Lithuanian"), ("lv", "Latvian" ), ("mk", "Macedonian"  ), ("no", "Norwegian"), ("fa", "Persian"  ),
	("pl", "Polish"   ), ("pt", "Portuguese"), ("ro", "Romanian"), ("ru", "Russian"     ), ("sr", "Serbian"  ), ("sk", "Slovak"   ),
	("sl", "Slovenian"), ("es", "Spanish"   ), ("sv", "Swedish" ), ("zh_TW", "Taiwanese"), ("tr", "Turkish"  ), ("uk", "Ukrainian"),
];

pub fn get_languages() -> GetLanguagesResponse {
	let mut res=GetLanguagesResponse{ language: String::from("fr"), ..Default::default() };
	for l in GUI_LANGUAGE.iter() {
		res.result.insert(String::from(l.1), String::from(l.0));
	}
	res
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetVecObjResponse<T> {
	pub result:Vec<T>,
	pub status: String,
	pub title: String
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct ThemeName {
	pub theme:String
}
pub fn get_themes(args: DomorustConfig) -> GetVecObjResponse<ThemeName> {
	let wwwroot = args.wwwroot;
	let paths = fs::read_dir(wwwroot.into_os_string().into_string().unwrap() + "/styles/").unwrap();
	let mut res=GetVecObjResponse::<ThemeName> {
		title: "GetThemes".to_string(),
		status: "OK".to_string(),
		result:vec![]
	};
	for dir in paths.flatten() {
		let theme_dir = dir.path();
		if theme_dir.is_dir() {
			res.result.push(ThemeName{theme:theme_dir.file_name().unwrap().to_str().unwrap().to_string()});
		}
	}
	res

	//check if current theme is found, if not, select default
	/*bool bFound = false;
	std::string sValue;
	if (m_sql.GetPreferencesVar("WebTheme", sValue))
		bFound = std::any_of(m_webthemes.begin(), m_webthemes.end(), [&](const std::string& s) { return s == sValue; });

	if (!bFound)
		m_sql.UpdatePreferencesVar("WebTheme", "default");*/
}

pub fn get_switch_types() -> RequestResult<String> {
	let mut st = RequestResult::<String>::new(
		"GetSwitchTypes",
		vec![]
	);
	for s in SWITCH_TYPES {
		st.result.push(s.to_string());
	}
	st
}
pub fn get_meter_types() -> RequestResult<String> {
	let mut st = RequestResult::<String>::new(
		"GetMeterTypes",
		vec![]
	);
	for s in METER_TYPES {
		st.result.push(s.to_string());
	}
	st
}

/* 
#[derive(Clone, Debug, Default, Serialize, FromSqlRow)]
pub struct SceneDevice
{
	pub Color:String,// : "{\"b\":0,\"cw\":60,\"g\":0,\"m\":2,\"r\":0,\"t\":195,\"ww\":195}",
	#[skip_field]
	pub Command:String,// : "Off",
	#[column_name("DeviceRowID")]
	pub DevID:String,// : "36",
	pub DevRealIdx:String,// : "36",
	pub ID:String,// : "1",
	pub Level:u8,// : 3,
	pub Name:String,// : "Salon C",
	pub OffDelay:u16,// : 0,
	pub OnDelay:u16,// : 0,
	pub Order:u16,// : 1,
	pub SubType:String,// : "RGBWW",
	pub Type:String,// : "Color Switch"
}
#[derive(Clone, Debug, Default, Serialize,FromSqlRow)]
pub struct Scene
{
	idx:String,// : "1"
	Name:String,// : "Mode Cin\u00e9ma",
	Description:String,// : "",
	Favorite:u8,// : 0,
	LastUpdate:String,// : "2024-08-02 10:45:07",
	OffAction:String,// : "",
	OnAction:String,// : "",
	Protected:bool,// : false,
	Status:String,// : "Mixed",
	Timers:String,// : "false",
	Type:String,// : "Scene",
	UsedByCamera:bool,// : false,
}

*/
