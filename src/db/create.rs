#![allow(non_upper_case_globals)]
use std::{error::Error, path::Path};

use domorust_models::{settings::{GPSCoord, Settings}, ToSqlRowFields};
use rusqlite::Connection;
use crate::db;
const DB_VERSION : u32 = 169;

const sqlCreateDeviceStatus : &str = r###"
CREATE TABLE IF NOT EXISTS [DeviceStatus] (
	[ID] INTEGER PRIMARY KEY, 
	[HardwareID] INTEGER NOT NULL, 
	[OrgHardwareID] INTEGER DEFAULT 0, 
	[DeviceID] VARCHAR(25) NOT NULL, 
	[Unit] INTEGER DEFAULT 0, 
	[Name] VARCHAR(100) DEFAULT Unknown, 
	[Used] INTEGER DEFAULT 0, 
	[Type] INTEGER NOT NULL, 
	[SubType] INTEGER NOT NULL, 
	[SwitchType] INTEGER DEFAULT 0, 
	[Favorite] INTEGER DEFAULT 0, 
	[SignalLevel] INTEGER DEFAULT 0, 
	[BatteryLevel] INTEGER DEFAULT 0, 
	[nValue] INTEGER DEFAULT 0, 
	[sValue] VARCHAR(200) DEFAULT null, 
	[LastUpdate] DATETIME DEFAULT (datetime('now','localtime')),
	[Order] INTEGER BIGINT(10) default 0, 
	[AddjValue] FLOAT DEFAULT 0, 
	[AddjMulti] FLOAT DEFAULT 1, 
	[AddjValue2] FLOAT DEFAULT 0, 
	[AddjMulti2] FLOAT DEFAULT 1, 
	[StrParam1] VARCHAR(200) DEFAULT '', 
	[StrParam2] VARCHAR(200) DEFAULT '', 
	[LastLevel] INTEGER DEFAULT 0, 
	[Protected] INTEGER DEFAULT 0, 
	[CustomImage] INTEGER DEFAULT 0, 
	[Description] VARCHAR(200) DEFAULT '', 
	[Options] TEXT DEFAULT null, 
	[Color] TEXT DEFAULT NULL
);"###;

const sqlCreateDeviceStatusTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS devicestatusupdate AFTER INSERT ON DeviceStatus
BEGIN
	UPDATE DeviceStatus SET [Order] = (SELECT MAX([Order]) FROM DeviceStatus)+1 WHERE DeviceStatus.ID = NEW.ID;
END;"###;

const sqlCreateLightingLog : &str = r###"
CREATE TABLE IF NOT EXISTS [LightingLog] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[nValue] INTEGER DEFAULT 0, 
	[sValue] VARCHAR(200), 
	[User] VARCHAR(100) DEFAULT (''), 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateSceneLog : &str = r###"
CREATE TABLE IF NOT EXISTS [SceneLog] (
	[SceneRowID] BIGINT(10) NOT NULL, 
	[nValue] INTEGER DEFAULT 0, 
	[User] VARCHAR(100) DEFAULT (''), 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreatePreferences  : &str = r###"
CREATE TABLE IF NOT EXISTS [Preferences] (
	[Key] VARCHAR(50) PRIMARY KEY, 
	[nValue] INTEGER DEFAULT 0, 
	[sValue] VARCHAR(200),
	[fValue] REAL
);"###;

const sqlCreateRain : &str = r###"
CREATE TABLE IF NOT EXISTS [Rain] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Total] FLOAT NOT NULL, 
	[Rate] INTEGER DEFAULT 0, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateRain_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [Rain_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Total] FLOAT NOT NULL, 
	[Rate] INTEGER DEFAULT 0, 
	[Date] DATE NOT NULL
);"###;

const sqlCreateTemperature : &str = r###"
CREATE TABLE IF NOT EXISTS [Temperature] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Temperature] FLOAT NOT NULL, 
	[Chill] FLOAT DEFAULT 0, 
	[Humidity] INTEGER DEFAULT 0, 
	[Barometer] INTEGER DEFAULT 0, 
	[DewPoint] FLOAT DEFAULT 0, 
	[SetPoint] FLOAT DEFAULT 0, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateTemperature_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [Temperature_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Temp_Min] FLOAT NOT NULL, 
	[Temp_Max] FLOAT NOT NULL, 
	[Temp_Avg] FLOAT DEFAULT 0, 
	[Chill_Min] FLOAT DEFAULT 0, 
	[Chill_Max] FLOAT, 
	[Humidity] INTEGER DEFAULT 0, 
	[Barometer] INTEGER DEFAULT 0, 
	[DewPoint] FLOAT DEFAULT 0, 
	[SetPoint_Min] FLOAT DEFAULT 0, 
	[SetPoint_Max] FLOAT DEFAULT 0, 
	[SetPoint_Avg] FLOAT DEFAULT 0, 
	[Date] DATE NOT NULL
);"###;

const sqlCreateTimers : &str = r###"
CREATE TABLE IF NOT EXISTS [Timers] (
	[ID] INTEGER PRIMARY KEY, 
	[Active] BOOLEAN DEFAULT true, 
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Date] DATE DEFAULT 0, 
	[Time] TIME NOT NULL, 
	[Type] INTEGER NOT NULL, 
	[Cmd] INTEGER NOT NULL, 
	[Level] INTEGER DEFAULT 15, 
	[Color] TEXT DEFAULT NULL, 
	[UseRandomness] INTEGER DEFAULT 0, 
	[TimerPlan] INTEGER DEFAULT 0, 
	[Days] INTEGER NOT NULL, 
	[Month] INTEGER DEFAULT 0, 
	[MDay] INTEGER DEFAULT 0, 
	[Occurence] INTEGER DEFAULT 0
);"###;

const sqlCreateUV : &str = r###"
CREATE TABLE IF NOT EXISTS [UV] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Level] FLOAT NOT NULL, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateUV_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [UV_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Level] FLOAT, 
	[Date] DATE NOT NULL
);"###;

const sqlCreateWind : &str = r###"
CREATE TABLE IF NOT EXISTS [Wind] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Direction] FLOAT NOT NULL, 
	[Speed] INTEGER NOT NULL, 
	[Gust] INTEGER NOT NULL, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateWind_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [Wind_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Direction] FLOAT NOT NULL, 
	[Speed_Min] INTEGER NOT NULL, 
	[Speed_Max] INTEGER NOT NULL, 
	[Gust_Min] INTEGER NOT NULL, 
	[Gust_Max] INTEGER NOT NULL, 
	[Date] DATE NOT NULL
);"###;

const sqlCreateMultiMeter : &str = r###"
CREATE TABLE IF NOT EXISTS [MultiMeter] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Value1] BIGINT NOT NULL, 
	[Value2] BIGINT DEFAULT 0, 
	[Value3] BIGINT DEFAULT 0, 
	[Value4] BIGINT DEFAULT 0, 
	[Value5] BIGINT DEFAULT 0, 
	[Value6] BIGINT DEFAULT 0, 
	[Price] FLOAT DEFAULT 0, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateMultiMeter_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [MultiMeter_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Value1] BIGINT NOT NULL, 
	[Value2] BIGINT NOT NULL, 
	[Value3] BIGINT NOT NULL, 
	[Value4] BIGINT NOT NULL, 
	[Value5] BIGINT NOT NULL, 
	[Value6] BIGINT NOT NULL, 
	[Counter1] BIGINT DEFAULT 0, 
	[Counter2] BIGINT DEFAULT 0, 
	[Counter3] BIGINT DEFAULT 0, 
	[Counter4] BIGINT DEFAULT 0, 
	[Price] FLOAT DEFAULT 0, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateNotifications : &str = r###"
CREATE TABLE IF NOT EXISTS [Notifications] (
	[ID] INTEGER PRIMARY KEY, 
	[Active] BOOLEAN DEFAULT true, 
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Params] VARCHAR(100), 
	[CustomMessage] VARCHAR(300) DEFAULT (''), 
	[CustomAction] VARCHAR(200) DEFAULT '', 
	[ActiveSystems] VARCHAR(200) DEFAULT (''), 
	[Priority] INTEGER default 0, 
	[SendAlways] INTEGER default 0, 
	[LastSend] DATETIME DEFAULT 0
);"###;

const sqlCreateHardware : &str = r###"
CREATE TABLE IF NOT EXISTS [Hardware] (
	[ID] INTEGER PRIMARY KEY, 
	[Name] VARCHAR(200) NOT NULL, 
	[Enabled] INTEGER DEFAULT 1, 
	[Type] INTEGER NOT NULL, 
	[LogLevel] INTEGER default 7, -- LOG_NORM + LOG_STATUS + LOG_ERROR
	[Address] VARCHAR(200), 
	[Port] INTEGER, 
	[SerialPort] TEXT DEFAULT (''), 
	[Username] VARCHAR(100), 
	[Password] VARCHAR(100), 
	[Extra] TEXT DEFAULT (''),
	[Mode1] CHAR DEFAULT 0, 
	[Mode2] CHAR DEFAULT 0, 
	[Mode3] CHAR DEFAULT 0, 
	[Mode4] CHAR DEFAULT 0, 
	[Mode5] CHAR DEFAULT 0, 
	[Mode6] CHAR DEFAULT 0, 
	[DataTimeout] INTEGER DEFAULT 0, 
	[Configuration] TEXT DEFAULT ('')
);"###;

const sqlCreateUsers : &str = r###"
CREATE TABLE IF NOT EXISTS [Users] (
	[ID] INTEGER PRIMARY KEY, 
	[Active] INTEGER NOT NULL DEFAULT 0, 
	[Username] VARCHAR(200) NOT NULL, 
	[Password] VARCHAR(200) NOT NULL, 
	[MFAsecret] VARCHAR(200) NULL, 
	[Rights] INTEGER DEFAULT 255, 
	[TabsEnabled] INTEGER DEFAULT 255, 
	[RemoteSharing] INTEGER DEFAULT 0
);"###;

const sqlCreateMeter : &str = r###"
CREATE TABLE IF NOT EXISTS [Meter] (
	[DeviceRowID] BIGINT NOT NULL, 
	[Value] BIGINT NOT NULL, 
	[Usage] INTEGER DEFAULT 0, 
	[Price] FLOAT DEFAULT 0, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateMeter_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [Meter_Calendar] (
	[DeviceRowID] BIGINT NOT NULL, 
	[Value] BIGINT NOT NULL, 
	[Counter] BIGINT DEFAULT 0, 
	[Price] FLOAT DEFAULT 0, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateLightSubDevices : &str = r###"
CREATE TABLE IF NOT EXISTS [LightSubDevices] (
	[ID] INTEGER PRIMARY KEY, 
	[DeviceRowID] INTEGER NOT NULL, 
	[ParentID] INTEGER NOT NULL
);"###;

const sqlCreateCameras : &str = r###"
CREATE TABLE IF NOT EXISTS [Cameras] (
	[ID] INTEGER PRIMARY KEY, 
	[Name] VARCHAR(200) NOT NULL, 
	[Enabled] INTEGER DEFAULT 1, 
	[Address] VARCHAR(200), 
	[Port] INTEGER, 
	[Protocol] INTEGER DEFAULT 0, 
	[AspectRatio] INTEGER DEFAULT 0, -- 0=4:3, 1=16:9
	[Username] VARCHAR(100) DEFAULT (''), 
	[Password] VARCHAR(100) DEFAULT (''), 
	[ImageURL] VARCHAR(200) DEFAULT ('')
);"###;

const sqlCreateCamerasActiveDevices : &str = r###"
CREATE TABLE IF NOT EXISTS [CamerasActiveDevices] (
	[ID] INTEGER PRIMARY KEY, 
	[CameraRowID] INTEGER NOT NULL, 
	[DevSceneType] INTEGER NOT NULL, 
	[DevSceneRowID] INTEGER NOT NULL, 
	[DevSceneWhen] INTEGER NOT NULL, 
	[DevSceneDelay] INTEGER NOT NULL
);"###;

const sqlCreatePlanMappings : &str = r###"
CREATE TABLE IF NOT EXISTS [DeviceToPlansMap] (
	[ID] INTEGER PRIMARY KEY, 
	[DeviceRowID] BIGINT NOT NULL, 
	[DevSceneType] INTEGER DEFAULT 0, 
	[PlanID] BIGINT NOT NULL, 
	[Order] INTEGER BIGINT(10) DEFAULT 0, 
	[XOffset] INTEGER default 0, 
	[YOffset] INTEGER default 0
);"###;

const sqlCreateDevicesToPlanStatusTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS deviceplantatusupdate AFTER INSERT ON DeviceToPlansMap
BEGIN
	UPDATE DeviceToPlansMap SET [Order] = (SELECT MAX([Order]) FROM DeviceToPlansMap)+1 WHERE DeviceToPlansMap.ID = NEW.ID;
END;"###;

const sqlCreatePlans : &str = r###"
CREATE TABLE IF NOT EXISTS [Plans] (
	[ID] INTEGER PRIMARY KEY, 
	[Order] INTEGER BIGINT(10) default 0, 
	[Name] VARCHAR(200) NOT NULL, 
	[FloorplanID] INTEGER default 0, 
	[Area] VARCHAR(200) DEFAULT ''
);"###;

const sqlCreatePlanOrderTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS planordertrigger AFTER INSERT ON Plans
BEGIN
	UPDATE Plans SET [Order] = (SELECT MAX([Order]) FROM Plans)+1 WHERE Plans.ID = NEW.ID;
END;"###;

const sqlCreateScenes : &str = r###"
CREATE TABLE IF NOT EXISTS [Scenes] (
	[ID] INTEGER PRIMARY KEY, 
	[Name] VARCHAR(100) NOT NULL, 
	[Favorite] INTEGER DEFAULT 0, 
	[Order] INTEGER BIGINT(10) default 0, 
	[nValue] INTEGER DEFAULT 0, 
	[SceneType] INTEGER DEFAULT 0, 
	[Protected] INTEGER DEFAULT 0, 
	[OnAction] VARCHAR(200) DEFAULT '', 
	[OffAction] VARCHAR(200) DEFAULT '', 
	[Description] VARCHAR(200) DEFAULT '', 
	[Activators] VARCHAR(200) DEFAULT '', 
	[LastUpdate] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateScenesTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS scenesupdate AFTER INSERT ON Scenes
BEGIN
	UPDATE Scenes SET [Order] = (SELECT MAX([Order]) FROM Scenes)+1 WHERE Scenes.ID = NEW.ID;
END;"###;

const sqlCreateSceneDevices : &str = r###"
CREATE TABLE IF NOT EXISTS [SceneDevices] (
	[ID] INTEGER PRIMARY KEY, 
	[Order] INTEGER BIGINT(10) default 0, 
	[SceneRowID] BIGINT NOT NULL, 
	[DeviceRowID] BIGINT NOT NULL, 
	[Cmd] INTEGER DEFAULT 1, 
	[Level] INTEGER DEFAULT 100, 
	[Color] TEXT DEFAULT NULL, 
	[OnDelay] INTEGER DEFAULT 0, 
	[OffDelay] INTEGER DEFAULT 0
);"###;

const sqlCreateSceneDeviceTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS scenedevicesupdate AFTER INSERT ON SceneDevices
BEGIN
	UPDATE SceneDevices SET [Order] = (SELECT MAX([Order]) FROM SceneDevices)+1 WHERE SceneDevices.ID = NEW.ID;
END;"###;

const sqlCreateTimerPlans : &str = r###"
CREATE TABLE IF NOT EXISTS [TimerPlans] (
	[ID] INTEGER PRIMARY KEY, 
	[Name] VARCHAR(200) NOT NULL
);"###;

const sqlCreateSceneTimers : &str = r###"
CREATE TABLE IF NOT EXISTS [SceneTimers] (
	[ID] INTEGER PRIMARY KEY, 
	[Active] BOOLEAN DEFAULT true, 
	[SceneRowID] BIGINT(10) NOT NULL, 
	[Date] DATE DEFAULT 0, 
	[Time] TIME NOT NULL, 
	[Type] INTEGER NOT NULL, 
	[Cmd] INTEGER NOT NULL, 
	[Level] INTEGER DEFAULT 15, 
	[UseRandomness] INTEGER DEFAULT 0, 
	[TimerPlan] INTEGER DEFAULT 0, 
	[Days] INTEGER NOT NULL, 
	[Month] INTEGER DEFAULT 0, 
	[MDay] INTEGER DEFAULT 0, 
	[Occurence] INTEGER DEFAULT 0
);"###;

const sqlCreateSetpointTimers : &str = r###"
CREATE TABLE IF NOT EXISTS [SetpointTimers] (
	[ID] INTEGER PRIMARY KEY, 
	[Active] BOOLEAN DEFAULT true, 
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Date] DATE DEFAULT 0, 
	[Time] TIME NOT NULL, 
	[Type] INTEGER NOT NULL, 
	[Temperature] FLOAT DEFAULT 0, 
	[TimerPlan] INTEGER DEFAULT 0, 
	[Days] INTEGER NOT NULL, 
	[Month] INTEGER DEFAULT 0, 
	[MDay] INTEGER DEFAULT 0, 
	[Occurence] INTEGER DEFAULT 0
);"###;

/*const sqlCreateSharedDevices : &str = r###"
CREATE TABLE IF NOT EXISTS [SharedDevices] (
	[ID] INTEGER PRIMARY KEY,  
	[SharedUserID] BIGINT NOT NULL, 
	[DeviceRowID] BIGINT NOT NULL, 
	[Favorite] INTEGER DEFAULT 0, 
	[Order] INTEGER BIGINT(10) default 0
);"###;

const sqlCreateSharedDevicesTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS shareddevicesupdate AFTER INSERT ON SharedDevices
BEGIN
	UPDATE SharedDevices SET [Order] = (SELECT MAX([Order]) FROM SharedDevices)+1 WHERE SharedDevices.ID = NEW.ID;
END;"###;*/

const sqlCreateEventMaster : &str = r###"
CREATE TABLE IF NOT EXISTS [EventMaster] (
	[ID] INTEGER PRIMARY KEY,  
	[Name] VARCHAR(200) NOT NULL, 
	[Interpreter] VARCHAR(10) DEFAULT 'Blockly', 
	[Type] VARCHAR(10) DEFAULT 'All', 
	[XMLStatement] TEXT NOT NULL, 
	[Status] INTEGER DEFAULT 0
);"###;

const sqlCreateEventRules : &str = r###"
CREATE TABLE IF NOT EXISTS [EventRules] (
	[ID] INTEGER PRIMARY KEY, 
	[EMID] INTEGER, 
	[Conditions] TEXT NOT NULL, 
	[Actions] TEXT NOT NULL, 
	[SequenceNo] INTEGER NOT NULL, 
	FOREIGN KEY (EMID) REFERENCES EventMaster(ID)
);"###;

const sqlCreateWOLNodes : &str = r###"
CREATE TABLE IF NOT EXISTS [WOLNodes] (
	[ID] INTEGER PRIMARY KEY, 
	[HardwareID] INTEGER NOT NULL, 
	[Name] VARCHAR(100) DEFAULT Unknown, 
	[MacAddress] VARCHAR(50) DEFAULT Unknown, 
	[Timeout] INTEGER DEFAULT 5
);"###;

const sqlCreatePercentage : &str = r###"
CREATE TABLE IF NOT EXISTS [Percentage] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Percentage] FLOAT NOT NULL, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreatePercentage_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [Percentage_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Percentage_Min] FLOAT NOT NULL, 
	[Percentage_Max] FLOAT NOT NULL, 
	[Percentage_Avg] FLOAT DEFAULT 0, 
	[Date] DATE NOT NULL
);"###;

const sqlCreateFan : &str = r###"
CREATE TABLE IF NOT EXISTS [Fan] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Speed] INTEGER NOT NULL, 
	[Date] DATETIME DEFAULT (datetime('now','localtime'))
);"###;

const sqlCreateFan_Calendar : &str = r###"
CREATE TABLE IF NOT EXISTS [Fan_Calendar] (
	[DeviceRowID] BIGINT(10) NOT NULL, 
	[Speed_Min] INTEGER NOT NULL, 
	[Speed_Max] INTEGER NOT NULL, 
	[Speed_Avg] INTEGER DEFAULT 0, 
	[Date] DATE NOT NULL
);"###;

const sqlCreateBackupLog : &str = r###"
CREATE TABLE IF NOT EXISTS [BackupLog] (
	[Key] VARCHAR(50) NOT NULL, 
	[nValue] INTEGER DEFAULT 0
);"###;

/*const sqlCreateEnOceanNodes : &str = r###"
CREATE TABLE IF NOT EXISTS [EnOceanNodes] (
	[ID] INTEGER PRIMARY KEY, 
	[HardwareID] INTEGER NOT NULL, 
	[NodeID] INTEGER NOT NULL, 
	[Name] VARCHAR(100) DEFAULT Unknown, 
	[ManufacturerID] INTEGER DEFAULT 0, 
	[RORG] INTEGER DEFAULT 0, 
	[Func] INTEGER DEFAULT 0, 
	[Type] INTEGER DEFAULT 0, 
	[Description] VARCHAR(100) DEFAULT Unknown, 
	[nValue] INTEGER DEFAULT 0
);"###;

const sqlCreatePushLink : &str = r###"
CREATE TABLE IF NOT EXISTS [PushLink] (
	[ID] INTEGER PRIMARY KEY, 
	[PushType] INTEGER, 
	[DeviceRowID] BIGINT NOT NULL, 
	[DelimitedValue] INTEGER DEFAULT 0, 
	[TargetType] INTEGER DEFAULT 0, 
	[TargetVariable] VARCHAR(100), 
	[TargetDeviceID] INTEGER, 
	[TargetProperty] VARCHAR(100), 
	[Enabled] INTEGER DEFAULT 1, 
	[IncludeUnit] INTEGER default 0
);"###;*/

const sqlCreateUserVariables : &str = r###"
CREATE TABLE IF NOT EXISTS [UserVariables] (
	[ID] INTEGER PRIMARY KEY, 
	[Name] VARCHAR(200), 
	[ValueType] INT NOT NULL, 
	[Value] TEXT, 
	[LastUpdate] DATETIME DEFAULT(datetime('now', 'localtime'))
);"###;

const sqlCreateFloorplans : &str = r###"
CREATE TABLE IF NOT EXISTS [Floorplans] (
	[ID] INTEGER PRIMARY KEY, 
	[Name] VARCHAR(200) NOT NULL, 
	[Image] BLOB, 
	[ScaleFactor] FLOAT DEFAULT 1.0, 
	[Order] INTEGER BIGINT(10) default 0
);"###;

const sqlCreateFloorplanOrderTrigger : &str = r###"
CREATE TRIGGER IF NOT EXISTS floorplanordertrigger AFTER INSERT ON Floorplans
BEGIN
	UPDATE Floorplans SET [Order] = (SELECT MAX([Order]) FROM Floorplans)+1 WHERE Floorplans.ID = NEW.ID;
END;"###;

const sqlCreateCustomImages : &str = r###"
CREATE TABLE IF NOT EXISTS [CustomImages](
	[ID] INTEGER PRIMARY KEY, 
	[Base] VARCHAR(80) NOT NULL, 
	[Name] VARCHAR(80) NOT NULL, 
	[Description] VARCHAR(80) NOT NULL, 
	[IconSmall] BLOB, 
	[IconOn] BLOB, 
	[IconOff] BLOB
);"###;

/*const sqlCreateMySensors : &str = r###"
CREATE TABLE IF NOT EXISTS [MySensors](
	[HardwareID] INTEGER NOT NULL,
	[ID] INTEGER NOT NULL,
	[Name] VARCHAR(100) DEFAULT Unknown,
	[SketchName] VARCHAR(100) DEFAULT Unknown,
	[SketchVersion] VARCHAR(40) DEFAULT(1.0)
);"###;

const sqlCreateMySensorsVariables : &str = r###"
CREATE TABLE IF NOT EXISTS [MySensorsVars](
	[HardwareID] INTEGER NOT NULL,
	[NodeID] INTEGER NOT NULL,
	[ChildID] INTEGER NOT NULL,
	[VarID] INTEGER NOT NULL,
	[Value] VARCHAR(100) NOT NULL
);"###;

const sqlCreateMySensorsChilds : &str = r###"
CREATE TABLE IF NOT EXISTS [MySensorsChilds](
	[HardwareID] INTEGER NOT NULL,
	[NodeID] INTEGER NOT NULL,
	[ChildID] INTEGER NOT NULL,
	[Name] VARCHAR(100) DEFAULT '',
	[Type] INTEGER NOT NULL,
	[UseAck] INTEGER DEFAULT 0,
	[AckTimeout] INTEGER DEFAULT 1200
);"###;

const sqlCreateToonDevices : &str = r###"
CREATE TABLE IF NOT EXISTS [ToonDevices](
	[HardwareID] INTEGER NOT NULL,
	[UUID] VARCHAR(100) NOT NULL
);"###;
*/
const sqlCreateUserSessions : &str = r###"
CREATE TABLE IF NOT EXISTS [UserSessions](
	[SessionID] VARCHAR(100) NOT NULL,
	[Username] VARCHAR(100) NOT NULL,
	[AuthToken] VARCHAR(100) UNIQUE NOT NULL,
	[ExpirationDate] DATETIME NOT NULL,
	[RemoteHost] VARCHAR(50) NOT NULL,
	[LastUpdate] DATETIME DEFAULT(datetime('now', 'localtime')),
	PRIMARY KEY([SessionID])
);"###;

const sqlCreateMobileDevices : &str = r###"
CREATE TABLE IF NOT EXISTS [MobileDevices](
	[ID] INTEGER PRIMARY KEY, 
	[Active] BOOLEAN DEFAULT false, 
	[Name] VARCHAR(100) DEFAULT '',
	[DeviceType] VARCHAR(100) DEFAULT '',
	[SenderID] TEXT NOT NULL,
	[UUID] TEXT NOT NULL, 
	[LastUpdate] DATETIME DEFAULT(datetime('now', 'localtime'))
);"###;

const sqlCreateApplications : &str = r###"
CREATE TABLE IF NOT EXISTS [Applications](
	[ID] INTEGER PRIMARY KEY, 
	[Active] BOOLEAN NOT NULL DEFAULT false, 
	[Public] BOOLEAN NOT NULL DEFAULT false, 
	[Applicationname] VARCHAR(100) DEFAULT '',
	[Secret] VARCHAR(100) DEFAULT '',
	[Pemfile] VARCHAR(100) DEFAULT '',
	[LastSeen] DATETIME DEFAULT NULL,
	[LastUpdate] DATETIME DEFAULT(datetime('now', 'localtime'))
);"###;

pub fn migrate_from_domoticz() -> Result<bool, Box<dyn Error>> {
	if !Path::new("domorust.db").exists() && Path::new("domoticz.db").exists() {
		std::fs::copy("domoticz.db", "domorust.db")?;
		let connection=Connection::open("domorust.db")?;
		let dbversion = crate::db::settings::get_setting_int("DB_Version")?;
		#[allow(clippy::comparison_chain)]
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
		}
	} else {
		return Ok(false)
	}
	let connection=Connection::open("domorust.db")?;
	connection.execute("ALTER TABLE Preferences ADD fValue REAL", [])?;
	let location = connection.query_row("Select sValue from Preferences WHERE Key='Location'", [], |row| {
		row.get::<usize, GPSCoord>(0)
	})?;
	connection.execute("INSERT INTO Preferences (Key, fValue) VALUES ('Latitude', ?1)", [location.Latitude])?;
	connection.execute("INSERT INTO Preferences (Key, fValue) VALUES ('Longitude', ?1)", [location.Longitude])?;
	//connection.execute("DELETE FROM Preferences WHERE Key='Location'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='TempHome'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='TempAway'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='TempComfort'", [])?;
	connection.execute("UPDATE Preferences SET fValue = CAST(sValue AS REAL) WHERE Key='DegreeDaysBaseTemperature'", [])?;
	connection.execute("DROP TABLE EnOceanNodes", [])?;
	connection.execute("DROP TABLE LightSubDevices", [])?;
	connection.execute("DROP TABLE MySensors", [])?;
	connection.execute("DROP TABLE MySensorsChilds", [])?;
	connection.execute("DROP TABLE MySensorsVars", [])?;
	connection.execute("DROP TABLE PushLink", [])?;
	connection.execute("DROP TABLE SharedDevices", [])?;
	connection.execute("DROP TABLE ToonDevices", [])?;
	connection.execute("DROP TABLE ZWaveNodes", [])?;
	
	Ok(true)
}
pub fn create_tables_if_needed() -> Result<(), rusqlite::Error> {
	let connection=Connection::open("domorust.db")?;
	connection.execute("BEGIN TRANSACTION;", [])?;
	connection.execute(sqlCreateDeviceStatus, [])?;
	connection.execute(sqlCreateDeviceStatusTrigger, [])?;
	connection.execute(sqlCreateLightingLog, [])?;
	connection.execute(sqlCreateSceneLog, [])?;
	connection.execute(sqlCreatePreferences, [])?;
	connection.execute(sqlCreateRain, [])?;
	connection.execute(sqlCreateRain_Calendar, [])?;
	connection.execute(sqlCreateTemperature, [])?;
	connection.execute(sqlCreateTemperature_Calendar, [])?;
	connection.execute(sqlCreateTimers, [])?;
	connection.execute(sqlCreateSetpointTimers, [])?;
	connection.execute(sqlCreateUV, [])?;
	connection.execute(sqlCreateUV_Calendar, [])?;
	connection.execute(sqlCreateWind, [])?;
	connection.execute(sqlCreateWind_Calendar, [])?;
	connection.execute(sqlCreateMeter, [])?;
	connection.execute(sqlCreateMeter_Calendar, [])?;
	connection.execute(sqlCreateMultiMeter, [])?;
	connection.execute(sqlCreateMultiMeter_Calendar, [])?;
	connection.execute(sqlCreateNotifications, [])?;
	connection.execute(sqlCreateHardware, [])?;
	connection.execute(sqlCreateUsers, [])?;
	connection.execute(sqlCreateLightSubDevices, [])?;
	connection.execute(sqlCreateCameras, [])?;
	connection.execute(sqlCreateCamerasActiveDevices, [])?;
	connection.execute(sqlCreatePlanMappings, [])?;
	connection.execute(sqlCreateDevicesToPlanStatusTrigger, [])?;
	connection.execute(sqlCreatePlans, [])?;
	connection.execute(sqlCreatePlanOrderTrigger, [])?;
	connection.execute(sqlCreateScenes, [])?;
	connection.execute(sqlCreateScenesTrigger, [])?;
	connection.execute(sqlCreateSceneDevices, [])?;
	connection.execute(sqlCreateSceneDeviceTrigger, [])?;
	connection.execute(sqlCreateTimerPlans, [])?;
	connection.execute(sqlCreateSceneTimers, [])?;
	//connection.execute(sqlCreateSharedDevices, [])?;
	//connection.execute(sqlCreateSharedDevicesTrigger, [])?;
	connection.execute(sqlCreateEventMaster, [])?;
	connection.execute(sqlCreateEventRules, [])?;
	connection.execute(sqlCreateWOLNodes, [])?;
	connection.execute(sqlCreatePercentage, [])?;
	connection.execute(sqlCreatePercentage_Calendar, [])?;
	connection.execute(sqlCreateFan, [])?;
	connection.execute(sqlCreateFan_Calendar, [])?;
	connection.execute(sqlCreateBackupLog, [])?;
	//connection.execute(sqlCreateEnOceanNodes, [])?;
	//connection.execute(sqlCreatePushLink, [])?;
	connection.execute(sqlCreateUserVariables, [])?;
	connection.execute(sqlCreateFloorplans, [])?;
	connection.execute(sqlCreateFloorplanOrderTrigger, [])?;
	connection.execute(sqlCreateCustomImages, [])?;
	//connection.execute(sqlCreateMySensors, [])?;
	//connection.execute(sqlCreateMySensorsVariables, [])?;
	//connection.execute(sqlCreateMySensorsChilds, [])?;
	//connection.execute(sqlCreateToonDevices, [])?;
	connection.execute(sqlCreateUserSessions, [])?;
	connection.execute(sqlCreateMobileDevices, [])?;
	connection.execute(sqlCreateApplications, [])?;
	//Add indexes to log tables
	connection.execute("create index if not exists ds_hduts_idx	on DeviceStatus(HardwareID, DeviceID, Unit, Type, SubType);", [])?;
	connection.execute("create index if not exists f_id_idx		on Fan(DeviceRowID);", [])?;
	connection.execute("create index if not exists f_id_date_idx   on Fan(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists fc_id_idx	   on Fan_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists fc_id_date_idx  on Fan_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists ll_id_idx	   on LightingLog(DeviceRowID);", [])?;
	connection.execute("create index if not exists ll_id_date_idx  on LightingLog(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists sl_id_idx	   on SceneLog(SceneRowID);", [])?;
	connection.execute("create index if not exists sl_id_date_idx  on SceneLog(SceneRowID, Date);", [])?;
	connection.execute("create index if not exists m_id_idx		on Meter(DeviceRowID);", [])?;
	connection.execute("create index if not exists m_id_date_idx   on Meter(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists mc_id_idx	   on Meter_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists mc_id_date_idx  on Meter_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists mm_id_idx	   on MultiMeter(DeviceRowID);", [])?;
	connection.execute("create index if not exists mm_id_date_idx  on MultiMeter(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists mmc_id_idx	  on MultiMeter_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists mmc_id_date_idx on MultiMeter_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists p_id_idx		on Percentage(DeviceRowID);", [])?;
	connection.execute("create index if not exists p_id_date_idx   on Percentage(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists pc_id_idx	   on Percentage_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists pc_id_date_idx  on Percentage_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists r_id_idx		on Rain(DeviceRowID);", [])?;
	connection.execute("create index if not exists r_id_date_idx   on Rain(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists rc_id_idx	   on Rain_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists rc_id_date_idx  on Rain_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists t_id_idx		on Temperature(DeviceRowID);", [])?;
	connection.execute("create index if not exists t_id_date_idx   on Temperature(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists tc_id_idx	   on Temperature_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists tc_id_date_idx  on Temperature_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists u_id_idx		on UV(DeviceRowID);", [])?;
	connection.execute("create index if not exists u_id_date_idx   on UV(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists uv_id_idx	   on UV_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists uv_id_date_idx  on UV_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists w_id_idx		on Wind(DeviceRowID);", [])?;
	connection.execute("create index if not exists w_id_date_idx   on Wind(DeviceRowID, Date);", [])?;
	connection.execute("create index if not exists wc_id_idx	   on Wind_Calendar(DeviceRowID);", [])?;
	connection.execute("create index if not exists wc_id_date_idx  on Wind_Calendar(DeviceRowID, Date);", [])?;
	connection.execute("END TRANSACTION;", [])?;
	Ok(())
}

pub fn set_default_values() -> Result<(), Box<dyn Error>> {
	let connection = Connection::open("domorust.db").unwrap();
	connection.query_row("PRAGMA journal_mode = WAL", [], |_row|{Ok(())})?;
	connection.query_row("PRAGMA synchronous = NORMAL", [], |_row|{Ok(())})?;
	connection.query_row("PRAGMA foreign_keys = ON", [], |_row|{Ok(())})?;
	connection.query_row("PRAGMA busy_timeout = 1000", [], |_row|{Ok(())})?;

	let result = connection.query_row("SELECT name FROM sqlite_master WHERE type='table' AND name='DeviceStatus'",[], |_row| {
		Ok(())
	});
	let bNewInstall = result.is_err() && result.unwrap_err() == rusqlite::Error::QueryReturnedNoRows;
	if bNewInstall {
		//place here actions that needs to be performed on new databases
		connection.execute("INSERT INTO Plans (Name) VALUES ('$Hidden Devices')", [])?;
		// Add hardware for internal use
		connection.execute("INSERT INTO Hardware (Name, Enabled, Type, Address, Port, Username, Password, Mode1, Mode2, Mode3, Mode4, Mode5, Mode6) VALUES ('Domoticz Internal',1, %d,'',1,'','',0,0,0,0,0,0)", [1])?;
		//connection.execute("INSERT INTO Users (Active, Username, Password, Rights, TabsEnabled) VALUES (1, '%s', '%s', %d, 0x1F)", (base64::encode("admin"), GenerateMD5Hash("domorust").c_str(), 2));
		//connection.execute("INSERT INTO Applications (Active, Public, Applicationname) VALUES (1, 1, 'domoticzUI')", []);
		//connection.execute("INSERT INTO Applications (Active, Public, Applicationname) VALUES (0, 0, 'domoticzMobileApp')", []);
	}
	db::settings::set_setting_int("DB_Version", DB_VERSION as i32)?;

	//Check preferences table for extreme sized sValues
	/*result = safe_query("SELECT Key FROM Preferences WHERE LENGTH(sValue) > 1000");
	if (!result.empty())
	{
		for (const auto &sd : result)
		{
			_log.Log(LOG_ERROR, "Preferences: sValue of Key %s has an extreme size. Please report on the forum", sd[0].c_str() );
		}
		_log.Log(LOG_STATUS, "Empty extreme sized sValue(s) in Preferences table to prevent future issues" );
		safe_query("UPDATE Preferences SET sValue ='' WHERE LENGTH(sValue) > 1000");
	}*/

	// Check if the default admin User password has been changed
	/*result = safe_query("SELECT Password FROM Users WHERE Username='%s'", [base64::encode("admin")]);
	if (!result.empty())
	{
		if (result[0][0] == GenerateMD5Hash("domorust"))
		{
			_log.Log(LOG_ERROR, "Default admin password has NOT been changed! Change it asap!");
		}
	}*/

	//Make sure we have some default preferences
	let settings = Settings::default();
	settings.write_instance(&connection)?;
	Ok(())
}