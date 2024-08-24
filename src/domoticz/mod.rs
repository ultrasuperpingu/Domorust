#![allow(non_snake_case, non_upper_case_globals)]
pub(crate) mod consts;
use self::consts::*;

pub fn is_light_switch(ty: u8) -> bool {
	#[allow(clippy::match_like_matches_macro)]
	match ty {
		pTypeLighting1 => true,
		pTypeLighting2 => true,
		pTypeLighting3 => true,
		pTypeLighting4 => true,
		pTypeLighting5 => true,
		pTypeLighting6 => true,
		pTypeFan => true,
		pTypeColorSwitch => true,
		pTypeSecurity1 => true,
		pTypeSecurity2 => true,
		pTypeEvohome => true,
		pTypeEvohomeRelay => true,
		pTypeCurtain => true,
		pTypeBlinds => true,
		pTypeRFY => true,
		pTypeChime => true,
		pTypeThermostat2 => true,
		pTypeThermostat3 => true,
		pTypeThermostat4 => true,
		pTypeRemote => true,
		pTypeRadiator1 => true,
		pTypeGeneralSwitch => true,
		pTypeHomeConfort => true,
		pTypeFS20 => true,
		pTypeHunter => true,
		pTypeDDxxxx => true,
		_ => false
	}
}


pub fn is_light_or_switch(dType: u8, dSubType: u8) -> bool
{
	match dType
	{
		pTypeLighting1 => true,
		pTypeLighting2 => true,
		pTypeLighting3 => true,
		pTypeLighting4 => true,
		pTypeLighting5 => true,
		pTypeLighting6 => true,
		pTypeFan => true,
		pTypeColorSwitch => true,
		pTypeSecurity1 => true,
		pTypeSecurity2 => true,
		pTypeCurtain => true,
		pTypeBlinds => true,
		pTypeChime => true,
		pTypeRFY => true,
		pTypeThermostat2 => true,
		pTypeThermostat3 => true,
		pTypeThermostat4 => true,
		pTypeRemote => true,
		pTypeGeneralSwitch => true,
		pTypeHomeConfort => true,
		pTypeFS20 => true,
		pTypeHunter => true,
		pTypeDDxxxx => true,
		pTypeRadiator1 => dSubType == sTypeSmartwaresSwitchRadiator,
		_ => false
	}
}

pub fn is_temp(dType: u8, dSubType: u8) -> bool
{
		(dType == pTypeTEMP_HUM)
		|| (dType == pTypeTEMP_HUM_BARO)
		|| (dType == pTypeTEMP)
		|| (dType == pTypeHUM)
		|| (dType == pTypeTEMP_BARO)
		|| (dType == pTypeEvohomeZone)
		|| (dType == pTypeEvohomeWater)
		|| ((dType == pTypeWIND) && (dSubType == sTypeWIND4))
		|| ((dType == pTypeUV) && (dSubType == sTypeUV3))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeSystemTemp))
		|| (dType == pTypeThermostat1)
		|| ((dType == pTypeRFXSensor) && (dSubType == sTypeRFXSensorTemp))
		|| (dType == pTypeRego6XXTemp)
}

pub fn is_weather(dType: u8, dSubType: u8) -> bool
{
		(dType == pTypeWIND)
		|| (dType == pTypeRAIN)
		|| (dType == pTypeTEMP_HUM_BARO)
		|| (dType == pTypeTEMP_BARO)
		|| (dType == pTypeUV)
		|| ((dType == pTypeGeneral) && (dSubType == sTypeVisibility))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeBaro))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeSolarRadiation))
}

pub fn is_utility(dType: u8, dSubType: u8) -> bool
{
		(dType == pTypeP1Power)
		|| (dType == pTypeP1Gas)
		|| ((dType == pTypeGeneral) && (dSubType == sTypeKwh))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeVoltage))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeCurrent))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeTextStatus))
		|| ((dType == pTypeGeneral) && (dSubType == sTypePercentage))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeCounterIncremental))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeManagedCounter))
		|| ((dType == pTypeRFXSensor) && (dSubType == sTypeRFXSensorVolt))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeWaterflow))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeCustom))
		|| ((dType == pTypeSetpoint) && (dSubType == sTypeSetpoint))
		|| ((dType == pTypeRFXSensor) && (dSubType == sTypeRFXSensorAD))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeAlert))
		|| ((dType == pTypeGeneral) && (dSubType == sTypePressure))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeSoilMoisture))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeLeafWetness))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeSoundLevel))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeFan))
		|| ((dType == pTypeGeneral) && (dSubType == sTypeDistance))
		|| (dType == pTypeLux)
		|| (dType == pTypeCURRENT)
		|| (dType == pTypeCURRENTENERGY)
		|| (dType == pTypeENERGY)
		|| (dType == pTypePOWER)
		|| (dType == pTypeYouLess)
		|| (dType == pTypeAirQuality)
		|| (dType == pTypeUsage)
		|| (dType == pTypeWEIGHT)
		|| (dType == pTypeRFXMeter)
		|| ((dType == pTypeRego6XXValue) && (dSubType == sTypeRego6XXCounter))
		|| ((dType == pTypeRadiator1) && (dSubType == sTypeSmartwares))
		|| ((dType == pTypeSetpoint) && (dSubType == sTypeSetpoint))
}

pub fn get_humidity_status(humidity: f32) -> &'static str {
	if humidity < 25.0 {
		"Dry"
	} else if humidity <= 60.0 {
		"Confortable"
	} else if humidity > 60.0 {
		"Wet"
	} else {
		"Normal"
	}
}