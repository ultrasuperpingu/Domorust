#![allow(non_snake_case, non_upper_case_globals)]
use std::collections::HashMap;

pub const sTypeTHBFloat : u8 = 0x10;	    // Weather Station
pub const sTypeWINDNoTemp : u8 = 0x30;	    // Weather Station
pub const sTypeWINDNoTempNoChill : u8 = 0x31; // Weather Station

pub const sTypeDomoticzSecurity : u8 = 0x83;
pub const sTypeSmartwaresSwitchRadiator : u8 = 0x84;

pub const sTypeRAINWU : u8 = 0x70;     // Weather Underground (Total rain reported, no counter)
pub const sTypeRAINByRate : u8 = 0x71; // DarkSky for example (Only rate, no total, no counter) rate in mm/hour x 10000, so all decimals will fit

pub const sTypeTH_LC_TC : u8 = 0xA0;    // La Cross Temp_Hum combined
pub const sTypeTEMP_SYSTEM : u8 = 0xA0; // Internal sensor
/*
pub const wsbaroforecast_heavy_snow : u8 = 0x00;
pub const wsbaroforecast_snow : u8 = 0x01;
pub const wsbaroforecast_heavy_rain : u8 = 0x02;
pub const wsbaroforecast_rain : u8 = 0x03;
pub const wsbaroforecast_cloudy : u8 = 0x04;
pub const wsbaroforecast_some_clouds : u8 = 0x05;
pub const wsbaroforecast_sunny : u8 = 0x06;
pub const wsbaroforecast_unknown : u8 = 0x07;
pub const wsbaroforecast_unstable : u8 = 0x08;
pub const wsbaroforecast_stable : u8 = 0x09;

pub const bmpbaroforecast_stable : u8 = 0x00;
pub const bmpbaroforecast_sunny : u8 = 0x01;
pub const bmpbaroforecast_cloudy : u8 = 0x02;
pub const bmpbaroforecast_unstable : u8 = 0x03;
pub const bmpbaroforecast_thunderstorm : u8 = 0x04;
pub const bmpbaroforecast_unknown : u8 = 0x05;
pub const bmpbaroforecast_rain : u8 = 0x06; // when forecast was cloudy and pressure is below 1010 we have 50%+ change of rain
*/
pub const pTypeSetpoint : u8 = 0xF2;
pub const sTypeSetpoint : u8 = 0x01;
pub const sTypeThermTemperature : u8 = 0x02;

pub const pTypeGeneral : u8 = 0xF3;
pub const sTypeVisibility : u8 = 0x01;
pub const sTypeSolarRadiation : u8 = 0x02;
pub const sTypeSoilMoisture : u8 = 0x03;
pub const sTypeLeafWetness : u8 = 0x04;
pub const sTypeSystemTemp : u8 = 0x05;
pub const sTypePercentage : u8 = 0x06;
pub const sTypeFan : u8 = 0x07;
pub const sTypeVoltage : u8 = 0x08;
pub const sTypePressure : u8 = 0x09;
pub const sTypeSetPoint : u8 = 0x10;
pub const sTypeTemperature : u8 = 0x11;
pub const sTypeTextStatus : u8 = 0x13;
pub const sTypeZWaveThermostatMode : u8 = 0x14;
pub const sTypeZWaveThermostatFanMode : u8 = 0x15;
pub const sTypeZWaveAlarm : u8 = 0x20;
pub const sTypeZWaveThermostatOperatingState : u8 = 0x23;
pub const sTypeAlert : u8 = 0x16;
pub const sTypeCurrent : u8 = 0x17;
pub const sTypeSoundLevel : u8 = 0x18;
pub const sTypeUV : u8 = 0x19;
pub const sTypeBaro : u8 = 0x1A;
pub const sTypeDistance : u8 = 0x1B;
pub const sTypeCounterIncremental : u8 = 0x1C;
pub const sTypeKwh : u8 = 0x1D;
pub const sTypeWaterflow : u8 = 0x1E;
pub const sTypeCustom : u8 = 0x1F;
pub const sTypeManagedCounter : u8 = 0x21;

// General Switch
pub const pTypeGeneralSwitch : u8 = 0xF4;
pub const sSwitchTypeX10 : u8 = 0x00;
pub const sSwitchTypeARC : u8 = 0x01;
pub const sSwitchTypeAB400D : u8 = 0x02;
pub const sSwitchTypeWaveman : u8 = 0x03;
pub const sSwitchTypeEMW200 : u8 = 0x04;
pub const sSwitchTypeIMPULS : u8 = 0x05;
pub const sSwitchTypeRisingSun : u8 = 0x06;
pub const sSwitchTypePhilips : u8 = 0x07;
pub const sSwitchTypeEnergenie : u8 = 0x08;
pub const sSwitchTypeEnergenie5 : u8 = 0x09;
pub const sSwitchTypeGDR2 : u8 = 0x0A;
pub const sSwitchTypeAC : u8 = 0x0B;
pub const sSwitchTypeHEU : u8 = 0x0C;
pub const sSwitchTypeANSLUT : u8 = 0x0D;
//pub const sSwitchTypeKambrook : u8 = 0x0E;
pub const sSwitchTypeKoppla : u8 = 0x0F;
pub const sSwitchTypePT2262 : u8 = 0x10;
pub const sSwitchTypeLightwaveRF : u8 = 0x11;
pub const sSwitchTypeEMW100 : u8 = 0x12;
pub const sSwitchTypeBBSB : u8 = 0x13;
pub const sSwitchTypeMDREMOTE : u8 = 0x14;
pub const sSwitchTypeRSL : u8 = 0x15;
pub const sSwitchTypeLivolo : u8 = 0x16;
pub const sSwitchTypeTRC02 : u8 = 0x17;
pub const sSwitchTypeAoke : u8 = 0x18;
pub const sSwitchTypeTRC02_2 : u8 = 0x19;
pub const sSwitchTypeEurodomest : u8 = 0x1A;
pub const sSwitchTypeLivoloAppliance : u8 = 0x1B;
pub const sSwitchTypeBlyss : u8 = 0x1C;
pub const sSwitchTypeByronSX : u8 = 0x1D;
pub const sSwitchTypeByronMP001 : u8 = 0x1E;
pub const sSwitchTypeSelectPlus : u8 = 0x1F;
pub const sSwitchTypeSelectPlus3 : u8 = 0x20;
pub const sSwitchTypeFA20 : u8 = 0x21;
pub const sSwitchTypeChuango : u8 = 0x22;
pub const sSwitchTypePlieger : u8 = 0x23;
pub const sSwitchTypeSilvercrest : u8 = 0x24;
pub const sSwitchTypeMertik : u8 = 0x25;
pub const sSwitchTypeHomeConfort : u8 = 0x26;
pub const sSwitchTypePowerfix : u8 = 0x27;
pub const sSwitchTypeTriState : u8 = 0x28;
pub const sSwitchTypeDeltronic : u8 = 0x29;
pub const sSwitchTypeFA500 : u8 = 0x30;
pub const sSwitchTypeHT12E : u8 = 0x31;
pub const sSwitchTypeEV1527 : u8 = 0x32;
pub const sSwitchTypeElmes : u8 = 0x33;
pub const sSwitchTypeAster : u8 = 0x34;
pub const sSwitchTypeSartano : u8 = 0x35;
pub const sSwitchTypeEurope : u8 = 0x36;
pub const sSwitchTypeAvidsen : u8 = 0x37;
pub const sSwitchTypeBofu : u8 = 0x38;
pub const sSwitchTypeBrel : u8 = 0x39;
pub const sSwitchTypeRTS : u8 = 0x3a;
pub const sSwitchTypeElroDB : u8 = 0x3b;
pub const sSwitchTypeDooya : u8 = 0x3c;
pub const sSwitchTypeUnitec : u8 = 0x3d;
pub const sSwitchTypeSelector : u8 = 0x3e;
pub const sSwitchTypeMaclean : u8 = 0x3f;
pub const sSwitchTypeR546 : u8 = 0x40;
pub const sSwitchTypeDiya : u8 = 0x41;
pub const sSwitchTypeX10secu : u8 = 0x42;
pub const sSwitchTypeAtlantic : u8 = 0x43;
pub const sSwitchTypeSilvercrestDB : u8 = 0x44;
pub const sSwitchTypeMedionDB : u8 = 0x45;
pub const sSwitchTypeVMC : u8 = 0x46;
pub const sSwitchTypeKeeloq : u8 = 0x47;
pub const sSwitchCustomSwitch : u8 = 0x48;
pub const sSwitchGeneralSwitch : u8 = 0x49;
pub const sSwitchTypeKoch : u8 = 0x4a;
pub const sSwitchTypeKingpin : u8 = 0x4b;
pub const sSwitchTypeFunkbus : u8 = 0x4c;
pub const sSwitchTypeNice : u8 = 0x4d;
pub const sSwitchTypeForest : u8 = 0x4e;
pub const sSwitchBlindsT1 : u8 = 0x4f;
pub const sSwitchMC145026 : u8 = 0x50;
pub const sSwitchLobeco : u8 = 0x51;
pub const sSwitchFriedland : u8 = 0x52;
pub const sSwitchBFT : u8 = 0x53;
pub const sSwitchNovatys : u8 = 0x54;
pub const sSwitchHalemeier : u8 = 0x55;
pub const sSwitchGaposa : u8 = 0x56;
pub const sSwitchMiLightv1 : u8 = 0x57;
pub const sSwitchMiLightv2 : u8 = 0x58;
pub const sSwitchHT6P20 : u8 = 0x59;
pub const sSwitchTypeDoitrand : u8 = 0x5a;
pub const sSwitchTypeWarema : u8 = 0x5b;
pub const sSwitchTypeAnsluta : u8 = 0x5c;
pub const sSwitchTypeLivcol : u8 = 0x5d;
pub const sSwitchTypeBosch : u8 = 0x5e;
pub const sSwitchTypeNingbo : u8 = 0x5f;
pub const sSwitchTypeDitec : u8 = 0x60;
pub const sSwitchTypeSteffen : u8 = 0x61;
pub const sSwitchTypeAlectoSA : u8 = 0x62;
pub const sSwitchTypeGPIOset : u8 = 0x63;
pub const sSwitchLightT1 : u8 = 0x64;
pub const sSwitchTypeKonigSec : u8 = 0x65;
pub const sSwitchTypeRM174RF : u8 = 0x66;
pub const sSwitchTypeLiwin : u8 = 0x67;
pub const sSwitchAuxiliaryT1 : u8 = 0x68;
pub const sSwitchBlindsT2 : u8 = 0x69;
pub const sSwitchLightT2 : u8 = 0x70;
pub const sSwitchContactT1 : u8 = 0x71;
pub const sSwitchTypeYW_Secu : u8 = 0x6a;
pub const sSwitchTypeMertik_GV60 : u8 = 0x6b;
pub const sSwitchTypeNingbo64 : u8 = 0x6c;
pub const sSwitchTypeX2D : u8 = 0x6d;
pub const sSwitchTypeHRCMotor : u8 = 0x6e;
pub const sSwitchTypeVelleman : u8 = 0x6f;
pub const sSwitchTypeRFCustom : u8 = 0x72;
pub const sSwitchTypeYW_Sensor : u8 = 0x73;
pub const sSwitchTypeLegrandcad : u8 = 0x74;
pub const sSwitchTypeSysfsGpio : u8 = 0x75;
pub const sSwitchTypeHager : u8 = 0x76;
pub const sSwitchTypeFaber : u8 = 0x77;
pub const sSwitchTypeDrayton : u8 = 0x78;
pub const sSwitchTypeV2Phoenix : u8 = 0x79;
/*pub const sSwitchTypeVisonic433 : u8 = 0x80;
pub const sSwitchTypeVisonic868 : u8 = 0x81;
pub const sSwitchTypeX2D433 : u8 = 0x82;
pub const sSwitchTypeX2D868 : u8 = 0x83;
pub const sSwitchTypeX2DShutter : u8 = 0x84;
pub const sSwitchTypeX2DElec : u8 = 0x85;
pub const sSwitchTypeX2DGas : u8 = 0x86;
pub const sSwitchTypeParrot : u8 = 0x87;
pub const sSwitchTypeKD101 : u8 = 0x88;
pub const sSwitchTypeFS20 : u8 = 0x89;*/

// Switch commands
/*pub const gswitch_sOff : u8 = 0x00;
pub const gswitch_sClose : u8 = 0x00;
pub const gswitch_sOn : u8 = 0x01;
pub const gswitch_sOpen : u8 = 0x01;
pub const gswitch_sSetLevel : u8 = 0x02;
pub const gswitch_sGroupOff : u8 = 0x03;
pub const gswitch_sGroupOn : u8 = 0x04;
pub const gswitch_sSetGroupLevel : u8 = 0x05;
pub const gswitch_sDim : u8 = 0x06;
pub const gswitch_sBright : u8 = 0x07;
pub const gswitch_sSound0 : u8 = 0x08;
pub const gswitch_sSound1 : u8 = 0x09;
pub const gswitch_sSound2 : u8 = 0x0A;
pub const gswitch_sSound3 : u8 = 0x0B;
pub const gswitch_sSound4 : u8 = 0x0C;
pub const gswitch_sSound5 : u8 = 0x0D;
pub const gswitch_sSound6 : u8 = 0x0E;
pub const gswitch_sSound7 : u8 = 0x0F;
pub const gswitch_sSound8 : u8 = 0x10;
pub const gswitch_sStop : u8 = 0x11;
pub const gswitch_sProgram : u8 = 0x12;
pub const gswitch_sPause : u8 = 0x13;
pub const gswitch_sPlay : u8 = 0x14;
pub const gswitch_sSetVolume : u8 = 0x15;
pub const gswitch_sPlayPlaylist : u8 = 0x16;
pub const gswitch_sPlayFavorites : u8 = 0x17;
pub const gswitch_sExecute : u8 = 0x18;
pub const gswitch_sColor : u8 = 0x19;
pub const gswitch_sDiscop : u8 = 0x1a;
pub const gswitch_sDiscom : u8 = 0x1b;

//
pub const gswitch_sToggle : u8 = 0xfe;*/

//--------------

pub const pTypeLux : u8 = 0xF6;
pub const sTypeLux : u8 = 0x01;

pub const pTypeTEMP_BARO : u8 = 0xF7;
pub const sTypeBMP085 : u8 = 0x01;

pub const pTypeUsage : u8 = 0xF8;
pub const sTypeElectric : u8 = 0x01;

pub const pTypeAirQuality : u8 = 0xF9;
pub const sTypeVoc : u8 = 0x01;

pub const pTypeP1Power : u8 = 0xFA;
pub const sTypeP1Power : u8 = 0x01;
//pub const mModeP1Norm : u8 = 0x00;
//pub const mModeP1Double : u8 = 0x01;

pub const pTypeP1Gas : u8 = 0xFB;
pub const sTypeP1Gas : u8 = 0x02;

pub const pTypeYouLess : u8 = 0xFC;
pub const sTypeYouLess : u8 = 0x01;

pub const pTypeRego6XXTemp : u8 = 0xFD;
pub const sTypeRego6XXTemp : u8 = 0x01;

pub const pTypeRego6XXValue : u8 = 0xFE;
pub const sTypeRego6XXStatus : u8 = 0x02;
pub const sTypeRego6XXCounter : u8 = 0x03;

// RFY2 (protocol v2)
pub const sTypeRFY2 : u8 = 0xFE;

// types for evohome
pub const pTypeEvohome : u8 = 0x45;
pub const sTypeEvohome : u8 = 0x00; // Controller

pub const pTypeEvohomeZone : u8 = 0x46; // Seems easier to define a new type here
pub const sTypeEvohomeZone : u8 = 0x00; // Actual temp zone

pub const pTypeEvohomeWater : u8 = 0x47; // Seems easier to define a new type here
pub const sTypeEvohomeWater : u8 = 0x00; // Hot water (Ideally this would just be a zone but for whatever reason evohome treats this differently)

pub const pTypeEvohomeRelay : u8 = 0x44; // Relay
pub const sTypeEvohomeRelay : u8 = 0x00;
pub const pTypeInterfaceControl : u8 = 0x00;
/*pub const sTypeInterfaceCommand : u8 = 0x00;
pub const cmdRESET : u8 = 0x00; // reset the receiver/transceiver
pub const cmdSTATUS : u8 = 0x02; // return firmware versions and configuration of the interface
pub const cmdSETMODE : u8 = 0x03; // set configuration of the interface

pub const cmdSAVE : u8 = 0x06; // save receiving modes of the receiver/transceiver in non-volatile memory
pub const cmdStartRec : u8 = 0x07; // start RFXtrx receiver
*/
pub const pTypeInterfaceMessage : u8 = 0x01;
/*pub const sTypeInterfaceResponse : u8 = 0x00;
pub const sTypeUnknownRFYremote : u8 = 0x01;
pub const sTypeExtError : u8 = 0x02;
pub const sTypeRFYremoteList : u8 = 0x03;
pub const sTypeASAremoteList : u8 = 0x04;
pub const sTypeCherubiniRemoteList : u8 = 0x05;
pub const sTypeOzrollRemoteList : u8 = 0x06;
pub const sTypeRecStarted : u8 = 0x07;
pub const sTypeInterfaceWrongCommand : u8 = 0xFF;
pub const trxType310 : u8 = 0x50;
pub const trxType315 : u8 = 0x51;
pub const recType43392 : u8 = 0x52;
pub const trxType43392 : u8 = 0x53;
pub const trxType868 : u8 = 0x55;

pub const FWtyperec : u8 = 0x0;
pub const FWtype1 : u8 = 0x1;
pub const FWtype2 : u8 = 0x2;
pub const FWtypeExt : u8 = 0x3;
pub const FWtypeExt2 : u8 = 0x4;
pub const FWtypePro1 : u8 = 0x5;
pub const FWtypePro2 : u8 = 0x6;
pub const FWtypeProXL1 : u8 = 0x10;
pub const FWtypeProXL2 : u8 = 0x13;
pub const FWtypeRFX433 : u8 = 0x14; //RFM69 433 firmware
pub const FWtypeRFX868 : u8 = 0x15; //RFM69 868 firmware
pub const FWtypeProXL95 : u8 = 0x16; //RFM95 firmware
pub const FWtypeRFX433XXL : u8 = 0x17; //RFU
pub const FWtypeRFX310 : u8 = 0x18; //RFXusb with 310MHz receiver module

//433 config bits
pub const msg3_AE : u8 = 0x01;			//AE Blyss
pub const msg3_RUBICSON : u8 = 0x02;		//Rubicson,Lacrosse, Banggood
pub const msg3_FINEOFFSET : u8 = 0x04;	//Fineoffset,Viking
pub const msg3_LIGHTING4 : u8 = 0x08;		//PT2262 and compatible
pub const msg3_RSL : u8 = 0x10;			//RSL,Revolt
pub const msg3_SX : u8 = 0x20;			//ByronSX,Selectplus
pub const msg3_IMAGINTRONIX : u8 = 0x40;	//Imagintronix,Opus
pub const msg3_undec : u8 = 0x80;			//display undecoded messages

pub const msg4_MERTIK : u8 = 0x01;		//Mertik maxitrol
pub const msg4_AD : u8 = 0x02;			//AD LightwaveRF
pub const msg4_HID : u8 = 0x04;			//Hideki
pub const msg4_LCROS : u8 = 0x08;			//LaCrosse
pub const msg4_LEGRAND : u8 = 0x10;		//Legrand CAD
pub const msg4_RFU : u8 = 0x20;			//RFU
pub const msg4_BLINDST0 : u8 = 0x40;		//Rollertrol,Hasta new
pub const msg4_BLINDST1 : u8 = 0x80;		//BlindsT1-4

pub const msg5_X10 : u8 = 0x01;			//X10
pub const msg5_ARC : u8 = 0x02;			//ARC
pub const msg5_AC : u8 = 0x04;			//AC
pub const msg5_HEU : u8 = 0x08;			//HomeEasy EU
pub const msg5_MEI : u8 = 0x10;			//Meiantech,Atlantic
pub const msg5_OREGON : u8 = 0x20;		//Oregon Scientific
pub const msg5_ATI : u8 = 0x40;			//ATI remotes
pub const msg5_VISONIC : u8 = 0x80;		//Visonic PowerCode

pub const msg6_KeeLoq : u8 = 0x01;		//Keeloq
pub const msg6_HC : u8 = 0x02;			//HomeConfort
pub const msg6_RFU2 : u8 = 0x04;			//RFU
pub const msg6_RFU3 : u8 = 0x08;			//RFU
pub const msg6_RFU4 : u8 = 0x10;			//RFU
pub const msg6_RFU5 : u8 = 0x20;			//RFU
pub const msg6_MCZ : u8 = 0x40;			//MCZ
pub const msg6_Funkbus : u8 = 0x80;		//Funkbus

//868 config bits
pub const msg3_868_RFU0 : u8 = 0x01;		//RFU
pub const msg3_868_DAVISAU : u8 = 0x02;	//Davis AU
pub const msg3_868_DAVISUS : u8 = 0x04;	//Davis US
pub const msg3_868_DAVISEU : u8 = 0x08;	//Davis EU
pub const msg3_868_LACROSSE : u8 = 0x10;  //LACROSSE
pub const msg3_868_FINEOFFSET : u8 = 0x20;	//FINEOFFSET
pub const msg3_868_ALECTO : u8 = 0x40;	//Alecto ACH2010
pub const msg3_868_UNDEC : u8 = 0x80;		//Enable undecoded

pub const msg4_868_EDISIO : u8 = 0x01;	//EDISIO
pub const msg4_868_LWRF : u8 = 0x02;		//LightwaveRF
pub const msg4_868_FS20 : u8 = 0x04;		//FS20
pub const msg4_868_RFU3 : u8 = 0x08;		//RFU
pub const msg4_868_RFU4 : u8 = 0x10;		//RFU
pub const msg4_868_RFU5 : u8 = 0x20;		//RFU
pub const msg4_868_RFU6 : u8 = 0x40;		//RFU
pub const msg4_868_RFU7 : u8 = 0x80;		//RFU

pub const msg5_868_RFU0 : u8 = 0x01;		//RFU
pub const msg5_868_RFU1 : u8 = 0x02;		//RFU
pub const msg5_868_RFU2 : u8 = 0x04;		//RFU
pub const msg5_868_RFU3 : u8 = 0x08;		//RFU
pub const msg5_868_PROGUARD : u8 = 0x10; //Proguard
pub const msg5_868_RFU5 : u8 = 0x20;		//RFU
pub const msg5_868_MEIANTECH : u8 = 0x40;	//Meiantech,Atlantic
pub const msg5_868_VISONIC : u8 = 0x80;	//Visonic

pub const msg6_868_KEELOQ : u8 = 0x01;    //KEELOQ
pub const msg6_868_HONCHIME : u8 = 0x02;	//Honeywell Chime
pub const msg6_868_RFU2 : u8 = 0x04;		//RFU
pub const msg6_868_RFU3 : u8 = 0x08;		//RFU
pub const msg6_868_ITHOHRU400 : u8 = 0x10; //Itho CVE-S,HRU400
pub const msg6_868_ORCON : u8 = 0x20;		//Orcon
pub const msg6_868_ITHOECO : u8 = 0x40;	//Itho CVE ECO RFT
pub const msg6_868_ITHO : u8 = 0x80;		//Itho CVE RFT

pub const msg7_868_DAVIS_ID0 : u8 = 0x01;    //Davis ID0
pub const msg7_868_DAVIS_ID1 : u8 = 0x02;    //Davis ID1
pub const msg7_868_DAVIS_ID2 : u8 = 0x04;    //Davis ID2
pub const msg7_868_RFU3 : u8 = 0x08;		//RFU
pub const msg7_868_RFU4 : u8 = 0x10;		//RFU
pub const msg7_868_RFU5 : u8 = 0x20;		//RFU
pub const msg7_868_RFU6 : u8 = 0x40;		//RFU
pub const msg7_868_RFU7 : u8 = 0x80;		//RFU
*/
pub const pTypeRecXmitMessage : u8 = 0x02;
//pub const sTypeReceiverLockError : u8 = 0x00;
//pub const sTypeTransmitterResponse : u8 = 0x01;

//undecoded types
pub const pTypeUndecoded : u8 = 0x03;
/*pub const sTypeUac : u8 = 0x00;
pub const sTypeUarc : u8 = 0x01;
pub const sTypeUati : u8 = 0x02;
pub const sTypeUhideki : u8 = 0x03;
pub const sTypeUlacrosse : u8 = 0x04;
pub const sTypeUad : u8 = 0x05;
pub const sTypeUmertik : u8 = 0x06;
pub const sTypeUoregon1 : u8 = 0x07;
pub const sTypeUoregon2 : u8 = 0x08;
pub const sTypeUoregon3 : u8 = 0x09;
pub const sTypeUproguard : u8 = 0x0A;
pub const sTypeUvisonic : u8 = 0x0B;
pub const sTypeUnec : u8 = 0x0C;
pub const sTypeUfs20 : u8 = 0x0D;
pub const sTypeUrsl : u8 = 0x0E;
pub const sTypeUblinds : u8 = 0x0F;
pub const sTypeUrubicson : u8 = 0x10;
pub const sTypeUae : u8 = 0x11;
pub const sTypeUfineoffset : u8 = 0x12;
pub const sTypeUrgb : u8 = 0x13;
pub const sTypeUrfy : u8 = 0x14;
pub const sTypeUselectplus : u8 = 0x15;
pub const sTypeUhomeconfort : u8 = 0x16;
pub const sTypeUedisio : u8 = 0x17;
pub const sTypeUhoneywell : u8 = 0x18;
pub const sTypeUfunkbus : u8 = 0x19;
pub const sTypeUbyronsx : u8 = 0x1A;
pub const sTypeUddxxxx : u8 = 0x1B;
pub const sTypeUitho : u8 = 0x1C;
pub const sTypeUorcon : u8 = 0x1D;
*/
//types for Lighting
pub const pTypeLighting1 : u8 = 0x10;
pub const sTypeX10 : u8 = 0x0;
pub const sTypeARC : u8 = 0x1;
pub const sTypeAB400D : u8 = 0x2;
pub const sTypeWaveman : u8 = 0x3;
pub const sTypeEMW200 : u8 = 0x4;
pub const sTypeIMPULS : u8 = 0x5;
pub const sTypeRisingSun : u8 = 0x6;
pub const sTypePhilips : u8 = 0x7;
pub const sTypeEnergenie : u8 = 0x8;
pub const sTypeEnergenie5 : u8 = 0x9;
pub const sTypeGDR2 : u8 = 0x0A;
pub const sTypeHQ : u8 = 0x0B;
pub const sTypeOase : u8 = 0x0C;

/*pub const light1_sOff : u8 = 0x0;
pub const light1_sOn : u8 = 0x1;
pub const light1_sDim : u8 = 0x2;
pub const light1_sBright : u8 = 0x3;
pub const light1_sProgram : u8 = 0x4;
pub const light1_sAllOff : u8 = 0x5;
pub const light1_sAllOn : u8 = 0x6;
pub const light1_sChime : u8 = 0x7;
*/
pub const pTypeLighting2 : u8 = 0x11;
pub const sTypeAC : u8 = 0x0;
pub const sTypeHEU : u8 = 0x1;
pub const sTypeANSLUT : u8 = 0x2;
/*pub const sTypeKambrook : u8 = 0x03;

pub const light2_sOff : u8 = 0x0;
pub const light2_sOn : u8 = 0x1;
pub const light2_sSetLevel : u8 = 0x2;
pub const light2_sGroupOff : u8 = 0x3;
pub const light2_sGroupOn : u8 = 0x4;
pub const light2_sSetGroupLevel : u8 = 0x5;*/

pub const pTypeLighting3 : u8 = 0x12;
pub const sTypeKoppla : u8 = 0x0;
/*pub const light3_sBright : u8 = 0x0;
pub const light3_sDim : u8 = 0x8;
pub const light3_sOn : u8 = 0x10;
pub const light3_sLevel1 : u8 = 0x11;
pub const light3_sLevel2 : u8 = 0x12;
pub const light3_sLevel3 : u8 = 0x13;
pub const light3_sLevel4 : u8 = 0x14;
pub const light3_sLevel5 : u8 = 0x15;
pub const light3_sLevel6 : u8 = 0x16;
pub const light3_sLevel7 : u8 = 0x17;
pub const light3_sLevel8 : u8 = 0x18;
pub const light3_sLevel9 : u8 = 0x19;
pub const light3_sOff : u8 = 0x1A;
pub const light3_sProgram : u8 = 0x1B;*/

pub const pTypeLighting4 : u8 = 0x13;
pub const sTypePT2262 : u8 = 0x0;

pub const pTypeLighting5 : u8 = 0x14;
pub const sTypeLightwaveRF : u8 = 0x0;
pub const sTypeEMW100 : u8 = 0x1;
pub const sTypeBBSB : u8 = 0x2;
pub const sTypeMDREMOTE : u8 = 0x03;
pub const sTypeRSL : u8 = 0x04;
pub const sTypeLivolo : u8 = 0x05;
pub const sTypeTRC02 : u8 = 0x06;
pub const sTypeAoke : u8 = 0x07;
pub const sTypeTRC02_2 : u8 = 0x08;
pub const sTypeEurodomest : u8 = 0x09;
pub const sTypeLivolo1to10 : u8 = 0x0A;
pub const sTypeRGB432W : u8 = 0x0B;
pub const sTypeMDREMOTE107 : u8 = 0x0C;
pub const sTypeLegrandCAD : u8 = 0x0D;
pub const sTypeAvantek : u8 = 0x0E;
pub const sTypeIT : u8 = 0x0F;
pub const sTypeMDREMOTE108 : u8 = 0x10;
pub const sTypeKangtai : u8 = 0x11;

/*pub const light5_sOff : u8 = 0x0;
pub const light5_sOn : u8 = 0x1;
pub const light5_sGroupOff : u8 = 0x2;
pub const light5_sLearn : u8 = 0x2;
pub const light5_sGroupOn : u8 = 0x3;
pub const light5_sMood1 : u8 = 0x3;
pub const light5_sMood2 : u8 = 0x4;
pub const light5_sMood3 : u8 = 0x5;
pub const light5_sMood4 : u8 = 0x6;
pub const light5_sMood5 : u8 = 0x7;
pub const light5_sUnlock : u8 = 0xA;
pub const light5_sLock : u8 = 0xB;
pub const light5_sAllLock : u8 = 0xC;
pub const light5_sClose : u8 = 0xD;
pub const light5_sStop : u8 = 0xE;
pub const light5_sOpen : u8 = 0xF;
pub const light5_sSetLevel : u8 = 0x10;
pub const light5_sColourPalette : u8 = 0x11;
pub const light5_sColourTone : u8 = 0x12;
pub const light5_sColourCycle : u8 = 0x13;
pub const light5_sPower : u8 = 0x0;
pub const light5_sLight : u8 = 0x1;
pub const light5_sBright : u8 = 0x2;
pub const light5_sDim : u8 = 0x3;
pub const light5_s100 : u8 = 0x4;
pub const light5_s50 : u8 = 0x5;
pub const light5_s25 : u8 = 0x6;
pub const light5_sModePlus : u8 = 0x7;
pub const light5_sSpeedMin : u8 = 0x8;
pub const light5_sSpeedPlus : u8 = 0x9;
pub const light5_sModeMin : u8 = 0xA;

//Livolo All off, used for all types
pub const light5_sLivoloAllOff : u8 = 0x00;

//Livolo 1-3 appliance modules
pub const light5_sLivoloGang1Toggle : u8 = 0x01;
pub const light5_sLivoloGang2Toggle : u8 = 0x02;
pub const light5_sLivoloGang3Toggle : u8 = 0x03;

//Livolo dimmer
//pub const light5_sLivoloToggle1 : u8 = 0x01;
pub const light5_sLivoloBright1 : u8 = 0x02;
pub const light5_sLivoloDim1 : u8 = 0x03;

//Livolo 1-10 appliance modules, 7 and 9 is a dimmer
pub const light5_sLivoloToggle1 : u8 = 0x01;
pub const light5_sLivoloToggle2 : u8 = 0x02;
pub const light5_sLivoloToggle3 : u8 = 0x03;
pub const light5_sLivoloToggle4 : u8 = 0x04;
pub const light5_sLivoloToggle5 : u8 = 0x05;
pub const light5_sLivoloToggle6 : u8 = 0x06;
pub const light5_sLivoloToggle7 : u8 = 0x07;
pub const light5_sLivoloBright7 : u8 = 0x08;
pub const light5_sLivoloDim7 : u8 = 0x09;
pub const light5_sLivoloToggle8 : u8 = 0x0A;
pub const light5_sLivoloToggle9 : u8 = 0x0B;
pub const light5_sLivoloBright9 : u8 = 0x0C;
pub const light5_sLivoloDim9 : u8 = 0x0D;
pub const light5_sLivoloToggle10 : u8 = 0x0E;
pub const light5_sLivoloScene1 : u8 = 0x0F;
pub const light5_sLivoloScene2 : u8 = 0x10;
pub const light5_sLivoloScene3 : u8 = 0x11;
pub const light5_sLivoloScene4 : u8 = 0x12;
pub const light5_sLivoloOkSet : u8 = 0x13;

pub const light5_sRGBoff : u8 = 0x00;
pub const light5_sRGBon : u8 = 0x01;
pub const light5_sRGBbright : u8 = 0x02;
pub const light5_sRGBdim : u8 = 0x03;
pub const light5_sRGBcolorplus : u8 = 0x04;
pub const light5_sRGBcolormin : u8 = 0x05;
pub const light5_sMD107_Power : u8 = 0x0;
pub const light5_sMD107_Bright : u8 = 0x1;
pub const light5_sMD107_Dim : u8 = 0x2;
pub const light5_sMD107_100 : u8 = 0x3;
pub const light5_sMD107_80 : u8 = 0x4;
pub const light5_sMD107_60 : u8 = 0x5;
pub const light5_sMD107_40 : u8 = 0x6;
pub const light5_sMD107_20 : u8 = 0x7;
pub const light5_sMD107_10 : u8 = 0x8;
pub const light5_sLegrandToggle : u8 = 0x00;*/

pub const pTypeLighting6 : u8 = 0x15;
pub const sTypeBlyss : u8 = 0x0;
/*pub const sTypeCuveo : u8 = 0x1;
pub const light6_sOn : u8 = 0x0;
pub const light6_sOff : u8 = 0x1;
pub const light6_sGroupOn : u8 = 0x2;
pub const light6_sGroupOff : u8 = 0x3;*/

pub const pTypeChime : u8 = 0x16;
pub const sTypeByronSX : u8 = 0x0;
pub const sTypeByronMP001 : u8 = 0x1;
pub const sTypeSelectPlus : u8 = 0x2;
pub const sTypeByronBY : u8 = 0x3;
pub const sTypeEnvivo : u8 = 0x4;
pub const sTypeAlfawise : u8 = 0x5;
/*pub const sType1byOne : u8 = 0x6;
pub const sTypeByronDBY : u8 = 0x7;
pub const chime_sound0 : u8 = 0x1;
pub const chime_sound1 : u8 = 0x3;
pub const chime_sound2 : u8 = 0x5;
pub const chime_sound3 : u8 = 0x9;
pub const chime_sound4 : u8 = 0xD;
pub const chime_sound5 : u8 = 0xE;
pub const chime_sound6 : u8 = 0x6;
pub const chime_sound7 : u8 = 0x2;*/

pub const pTypeFan : u8 = 0x17;
pub const sTypeSiemensSF01 : u8 = 0x0;
pub const sTypeItho : u8 = 0x1;
pub const sTypeLucciAir : u8 = 0x2;
pub const sTypeSeavTXS4 : u8 = 0x3;
pub const sTypeWestinghouse : u8 = 0x4;
pub const sTypeLucciAirDC : u8 = 0x5;
pub const sTypeCasafan : u8 = 0x6;
pub const sTypeFT1211R : u8 = 0x7;
pub const sTypeFalmec : u8 = 0x8;
pub const sTypeLucciAirDCII : u8 = 0x9;
pub const sTypeIthoECO : u8 = 0xA;
pub const sTypeNovy : u8 = 0xB;
pub const sTypeOrcon : u8 = 0xC;
pub const sTypeIthoHRU400 : u8 = 0x0D;

/*pub const fan_sTimer : u8 = 0x1;
pub const fan_sMin : u8 = 0x2;
pub const fan_sLearn : u8 = 0x3;
pub const fan_sPlus : u8 = 0x4;
pub const fan_sConfirm : u8 = 0x5;
pub const fan_sLight : u8 = 0x6;

pub const fan_Itho1 : u8 = 0x1;
pub const fan_Itho2 : u8 = 0x2;
pub const fan_Itho3 : u8 = 0x3;
pub const fan_IthoTimer : u8 = 0x4;
pub const fan_IthoNotAtHome : u8 = 0x5;
pub const fan_IthoLearn : u8 = 0x6;
pub const fan_IthoEraseAll : u8 = 0x7;

pub const fan_LucciHi : u8 = 0x1;
pub const fan_LucciMed : u8 = 0x2;
pub const fan_LucciLow : u8 = 0x3;
pub const fan_LucciOff : u8 = 0x4;
pub const fan_LucciLight : u8 = 0x5;

pub const fan_T1 : u8 = 0x1;
pub const fan_T2 : u8 = 0x2;
pub const fan_T3 : u8 = 0x3;
pub const fan_T4 : u8 = 0x4;

pub const fan_WestinghouseHi : u8 = 0x1;
pub const fan_WestinghouseMed : u8 = 0x2;
pub const fan_WestinghouseLow : u8 = 0x3;
pub const fan_WestinghouseOff : u8 = 0x4;
pub const fan_WestinghouseLight : u8 = 0x5;

pub const fan_LucciDCPower : u8 = 0x1;
pub const fan_LucciDCPlus : u8 = 0x2;
pub const fan_LucciDCMin : u8 = 0x3;
pub const fan_LucciDCLight : u8 = 0x4;
pub const fan_LucciDCReverse : u8 = 0x5;
pub const fan_LucciDCNaturalflow : u8 = 0x6;
pub const fan_LucciDCPair : u8 = 0x7;
pub const LucciDCSpeed1 : u8 = 0x8;
pub const LucciDCSpeed2 : u8 = 0x9;
pub const LucciDCSpeed3 : u8 = 0xA;
pub const LucciDCSpeed4 : u8 = 0xB;
pub const LucciDCSpeed5 : u8 = 0xC;
pub const LucciDCSpeed6 : u8 = 0xD;

pub const fan_CasafanHi : u8 = 0x1;
pub const fan_CasafanMed : u8 = 0x2;
pub const fan_CasafanLow : u8 = 0x3;
pub const fan_CasafanOff : u8 = 0x4;
pub const fan_CasafanLight : u8 = 0x5;
pub const fan_FT1211Rpower : u8 = 0x1;
pub const fan_FT1211Rlight : u8 = 0x2;
pub const fan_FT1211R1 : u8 = 0x3;
pub const fan_FT1211R2 : u8 = 0x4;
pub const fan_FT1211R3 : u8 = 0x5;
pub const fan_FT1211R4 : u8 = 0x6;
pub const fan_FT1211R5 : u8 = 0x7;
pub const fan_FT1211Rfr : u8 = 0x8;
pub const fan_FT1211R1H : u8 = 0x9;
pub const fan_FT1211R4H : u8 = 0xA;
pub const fan_FT1211R8H : u8 = 0xB;
pub const fan_FalmecPower : u8 = 0x1;
pub const fan_FalmecSpeed1 : u8 = 0x2;
pub const fan_FalmecSpeed2 : u8 = 0x3;
pub const fan_FalmecSpeed3 : u8 = 0x4;
pub const fan_FalmecSpeed4 : u8 = 0x5;
pub const fan_FalmecTimer1 : u8 = 0x6;
pub const fan_FalmecTimer2 : u8 = 0x7;
pub const fan_FalmecTimer3 : u8 = 0x8;
pub const fan_FalmecTimer4 : u8 = 0x9;
pub const fan_FalmecLightOn : u8 = 0xA;
pub const fan_FalmecLightOff : u8 = 0xB;
pub const fan_LucciDCIIOff : u8 = 0x1;
pub const fan_LucciDCII1 : u8 = 0x2;
pub const fan_LucciDCII2 : u8 = 0x3;
pub const fan_LucciDCII3 : u8 = 0x4;
pub const fan_LucciDCII4 : u8 = 0x5;
pub const fan_LucciDCII5 : u8 = 0x6;
pub const fan_LucciDCII6 : u8 = 0x7;
pub const fan_LucciDCIILight : u8 = 0x8;
pub const fan_LucciDCIIReverse : u8 = 0x9;

pub const fan_IthoECOlow : u8 = 0x1;
pub const fan_IthoECOmedium : u8 = 0x2;
pub const fan_IthoECOhigh : u8 = 0x3;
pub const fan_IthoECOtimer1 : u8 = 0x4;
pub const fan_IthoECOtimer2 : u8 = 0x5;
pub const fan_IthoECOtimer3 : u8 = 0x6;
pub const fan_IthoECOaway : u8 = 0x7;
pub const fan_IthoECOjoin : u8 = 0x8;
pub const fan_IthoECOleave : u8 = 0x9;
pub const fan_IthoECOtemp : u8 = 0xA;
pub const fan_IthoECOco2 : u8 = 0xB;
pub const fan_IthoECObattery : u8 = 0xC;

pub const fan_NovyPower : u8 = 0x1;
pub const fan_NovyPlus : u8 = 0x2;
pub const fan_NovyMin : u8 = 0x3;
pub const fan_NovyLight : u8 = 0x4;
pub const fan_NovyLearn : u8 = 0x5;
pub const fan_NovyFilter : u8 = 0x6;
pub const fan_NovyMood : u8 = 0x7;

pub const fan_Orconlow : u8 = 0x1;
pub const fan_Orconmedium : u8 = 0x2;
pub const fan_Orconhigh : u8 = 0x3;
pub const fan_Orcontimer1 : u8 = 0x4;
pub const fan_Orcontimer2 : u8 = 0x5;
pub const fan_Orcontimer3 : u8 = 0x6;
pub const fan_Orconauto : u8 = 0x7;
pub const fan_Orconaway : u8 = 0x8;
pub const fan_Orconjoin : u8 = 0x9;
pub const fan_Orconleave : u8 = 0xA;
pub const fan_Orpub constate : u8 = 0xB;
pub const fan_Orcontemp : u8 = 0xC;
pub const fan_Orconco2 : u8 = 0xD;
pub const fan_Orconbattery : u8 = 0xE;
pub const fan_Orconfilter : u8 = 0xF;
pub const fan_Orconpresence : u8 = 0x10;
pub const fan_Orconspeed : u8 = 0x11;
pub const fan_Orpub constatus : u8 = 0x12;*/

//types for Curtain
pub const pTypeCurtain : u8 = 0x18;
pub const sTypeHarrison : u8 = 0x0;
/*pub const curtain_sOpen : u8 = 0x0;
pub const curtain_sClose : u8 = 0x1;
pub const curtain_sStop : u8 = 0x2;
pub const curtain_sProgram : u8 = 0x3;*/

//types for Blinds
pub const pTypeBlinds : u8 = 0x19;
pub const sTypeBlindsT0 : u8 = 0x0;	//RollerTrol, Hasta new
pub const sTypeBlindsT1 : u8 = 0x1;	//Hasta old
pub const sTypeBlindsT2 : u8 = 0x2;	//A-OK RF01
pub const sTypeBlindsT3 : u8 = 0x3;	//A-OK AC114
pub const sTypeBlindsT4 : u8 = 0x4;	//RAEX YR1326
pub const sTypeBlindsT5 : u8 = 0x5;	//Media Mount
pub const sTypeBlindsT6 : u8 = 0x6;	//DC106, YOOHA, Rohrmotor24 RMF
pub const sTypeBlindsT7 : u8 = 0x7;	//Forest
pub const sTypeBlindsT8 : u8 = 0x8;	//Chamberlain CS4330CN
pub const sTypeBlindsT9 : u8 = 0x9;	//Sunpery
pub const sTypeBlindsT10 : u8 = 0xA;	//Dolat DLM-1
pub const sTypeBlindsT11 : u8 = 0xB;	//ASP
pub const sTypeBlindsT12 : u8 = 0xC;	//Confexx
pub const sTypeBlindsT13 : u8 = 0xD;	//Screenline
pub const sTypeBlindsT14 : u8 = 0xE;	//Hualite
pub const sTypeBlindsT15 : u8 = 0xF;	//Motostar
pub const sTypeBlindsT16 : u8 = 0x10;	//Zemismart
pub const sTypeBlindsT17 : u8 = 0x11;	//Gaposa
pub const sTypeBlindsT18 : u8 = 0x12;	//Cherubini
//pub const sTypeBlindsT19 : u8 = 0x13;	//Louvolite vertical
//pub const sTypeBlindsT20 : u8 = 0x14;	//Ozroll E-Trans

/*pub const blinds_sOpen : u8 = 0x0;
pub const blinds_sClose : u8 = 0x1;
pub const blinds_sStop : u8 = 0x2;
pub const blinds_sConfirm : u8 = 0x3;
pub const blinds_sLimit : u8 = 0x4;
pub const blinds_slowerLimit : u8 = 0x5;
pub const blinds_sDeleteLimits : u8 = 0x6;
pub const blinds_sChangeDirection : u8 = 0x7;
pub const blinds_sLeft : u8 = 0x8;
pub const blinds_sRight : u8 = 0x9;
pub const blinds_s6Im : u8 = 0x4;
pub const blinds_s6Light : u8 = 0x5;
pub const blinds_s9ChangeDirection : u8 = 0x6;
pub const blinds_s9ImA : u8 = 0x7;
pub const blinds_s9ImCenter : u8 = 0x8;
pub const blinds_s9ImB : u8 = 0x9;
pub const blinds_s9EraseCurrentCh : u8 = 0xA;
pub const blinds_s9EraseAllCh : u8 = 0xB;
pub const blinds_s10LearnMaster : u8 = 0x4;
pub const blinds_s10EraseCurrentCh : u8 = 0x5;
pub const blinds_s10ChangeDirection : u8 = 0x6;
pub const blinds_s13anglePlus : u8 = 0x4;
pub const blinds_s13angleMinus : u8 = 0x5;
pub const blinds_s16EraseCurrentCh : u8 = 0x4;
pub const blinds_s16ChangeDirection : u8 = 0x5;
pub const blinds_s17Intermediate : u8 = 0x5;
pub const blinds_s17EraseAllCh : u8 = 0x6;
pub const blinds_s18Intermediate : u8 = 0x4;
pub const blinds_s18ListRemotes : u8 = 0x5;
pub const blinds_s18EraseThis : u8 = 0x6;
pub const blinds_s18EraseAll : u8 = 0x7;
pub const blinds_s19CloseCW : u8 = 0x0;
pub const blinds_s19CloseCCW : u8 = 0x1;
pub const blinds_s19Open45 : u8 = 0x2;
pub const blinds_s19Open90 : u8 = 0x3;
pub const blinds_s19Open135 : u8 = 0x4;
pub const blinds_s19Confirm : u8 = 0x5;
pub const blinds_s20ListRemotes : u8 = 0x4;
pub const blinds_s20EraseThis : u8 = 0x5;
pub const blinds_s20EraseAll : u8 = 0x6;*/


//types for RFY
pub const pTypeRFY : u8 = 0x1A;
pub const sTypeRFY : u8 = 0x0;	//RFY
pub const sTypeRFYext : u8 = 0x1;	//RFY extended
pub const sTypeASA : u8 = 0x3;	//ASA
/*pub const rfy_sStop : u8 = 0x0;
pub const rfy_sUp : u8 = 0x1;
pub const rfy_sUpStop : u8 = 0x2;
pub const rfy_sDown : u8 = 0x3;
pub const rfy_sDownStop : u8 = 0x4;
pub const rfy_sUpDown : u8 = 0x5;
pub const rfy_sListRemotes : u8 = 0x6;
pub const rfy_sProgram : u8 = 0x7;
pub const rfy_s2SecProgram : u8 = 0x8;
pub const rfy_s7SecProgram : u8 = 0x9;
pub const rfy_s2SecStop : u8 = 0xA;
pub const rfy_s5SecStop : u8 = 0xB;
pub const rfy_s5SecUpDown : u8 = 0xC;
pub const rfy_sEraseThis : u8 = 0xD;
pub const rfy_sEraseAll : u8 = 0xE;
pub const rfy_s05SecUp : u8 = 0xF;
pub const rfy_s05SecDown : u8 = 0x10;
pub const rfy_s2SecUp : u8 = 0x11;
pub const rfy_s2SecDown : u8 = 0x12;
pub const rfy_sEnableSunWind : u8 = 0x13;
pub const rfy_sDisableSun : u8 = 0x14;*/

//types for Home Confort
pub const pTypeHomeConfort : u8 = 0x1B;
pub const sTypeHomeConfortTEL010 : u8 = 0x0;
/*pub const HomeConfort_sOff : u8 = 0x0;
pub const HomeConfort_sOn : u8 = 0x1;
pub const HomeConfort_sGroupOff : u8 = 0x2;
pub const HomeConfort_sGroupOn : u8 = 0x3;

//types for Funkbus
pub const pTypeFunkbus : u8 = 0x1E;
pub const sTypeFunkbusRemoteGira : u8 = 0x00;
pub const sTypeFunkbusRemoteInsta : u8 = 0x01;
pub const Funkbus_sChannelMin : u8 = 0x00;
pub const Funkbus_sChannelPlus : u8 = 0x01;
pub const Funkbus_sAllOff : u8 = 0x02;
pub const Funkbus_sAllOn : u8 = 0x03;
pub const Funkbus_sScene : u8 = 0x04;
pub const Funkbus_sMasterMin : u8 = 0x05;
pub const Funkbus_sMasterPlus : u8 = 0x06;*/

//types for Hunter Fan
pub const pTypeHunter : u8 = 0x1F;
pub const sTypeHunterfan : u8 = 0x00;
/*pub const HunterOff : u8 = 0x1;
pub const HunterLight : u8 = 0x2;
pub const HunterSpeed1 : u8 = 0x3;
pub const HunterSpeed2 : u8 = 0x4;
pub const HunterSpeed3 : u8 = 0x5;
pub const HunterProgram : u8 = 0x6;*/

//types for Edisio
/*pub const pTypeEdisio : u8 = 0x1C;
pub const sTypeEdisioGateway : u8 = 0x00;
pub const sTypeEdisioEmitter : u8 = 0x01;
pub const sTypeEdisioEmit230 : u8 = 0x02;
pub const sTypeEdisioMotion : u8 = 0x03;
pub const sTypeEdisioDoor : u8 = 0x04;
pub const sTypeEdisioTemp : u8 = 0x05;
pub const sTypeEdisioLighting : u8 = 0x06;
pub const sTypeEdisioDimmer : u8 = 0x07;
pub const sTypeEdisioLightOnOff : u8 = 0x08;
pub const sTypeEdisioShutter : u8 = 0x09;
pub const sTypeEdisioShutterOSC : u8 = 0x0A;
pub const sTypeEdisioHeatCool : u8 = 0x0B;
pub const sTypeEdisioPilot : u8 = 0x0C;

pub const EdisioOff : u8 = 0x00;
pub const EdisioOn : u8 = 0x01;
pub const EdisioToggle : u8 = 0x02;
pub const EdisioSetLevel : u8 = 0x03;
pub const EdisioBright : u8 = 0x04;
pub const EdisioDim : u8 = 0x05;
pub const EdisioToggleDim : u8 = 0x06;
pub const EdisioStopDim : u8 = 0x07;
pub const EdisioRGB : u8 = 0x08;
pub const EdisioLearn : u8 = 0x09;
pub const EdisioShutterOpen : u8 = 0x0A;
pub const EdisioShutterStop : u8 = 0x0B;
pub const EdisioShutterClose : u8 = 0x0C;
pub const EdisioContactNormal : u8 = 0x0D;
pub const EdisioContactAlert : u8 = 0x0E;

//types for Honeywell ActivLink
pub const pTypeHoneywell_AL : u8 = 0x1D;
pub const sTypeSeries5 : u8 = 0x00;
pub const sTypePIR : u8 = 0x01;
*/
//types for Security1
pub const pTypeSecurity1 : u8 = 0x20;
pub const sTypeSecX10 : u8 = 0x0;				//X10 security
pub const sTypeSecX10M : u8 = 0x1;			//X10 security motion
pub const sTypeSecX10R : u8 = 0x2;			//X10 security remote
pub const sTypeKD101 : u8 = 0x3;				//KD101 smoke detector
pub const sTypePowercodeSensor : u8 = 0x04;	//Visonic PowerCode sensor - primary contact
pub const sTypePowercodeMotion : u8 = 0x05;	//Visonic PowerCode motion
pub const sTypeCodesecure : u8 = 0x06;		//Visonic CodeSecure
pub const sTypePowercodeAux : u8 = 0x07;		//Visonic PowerCode sensor - auxiliary contact
pub const sTypeMeiantech : u8 = 0x8;			//Meiantech
pub const sTypeSA30 : u8 = 0x9;				//SA30 smoke detector
pub const sTypeRM174RF : u8 = 0xA;			//RM174RF smoke detector

//status security
/*pub const sStatusNormal : u8 = 0x0;
pub const sStatusNormalDelayed : u8 = 0x1;
pub const sStatusAlarm : u8 = 0x2;
pub const sStatusAlarmDelayed : u8 = 0x3;
pub const sStatusMotion : u8 = 0x4;
pub const sStatusNoMotion : u8 = 0x5;
pub const sStatusPanic : u8 = 0x6;
pub const sStatusPanicOff : u8 = 0x7;
pub const sStatusIRbeam : u8 = 0x8;
pub const sStatusArmAway : u8 = 0x9;
pub const sStatusArmAwayDelayed : u8 = 0xA;
pub const sStatusArmHome : u8 = 0xB;
pub const sStatusArmHomeDelayed : u8 = 0xC;
pub const sStatusDisarm : u8 = 0xD;
pub const sStatusLightOff : u8 = 0x10;
pub const sStatusLightOn : u8 = 0x11;
pub const sStatusLight2Off : u8 = 0x12;
pub const sStatusLight2On : u8 = 0x13;
pub const sStatusDark : u8 = 0x14;
pub const sStatusLight : u8 = 0x15;
pub const sStatusBatLow : u8 = 0x16;
pub const sStatusPairKD101 : u8 = 0x17;
pub const sStatusNormalTamper : u8 = 0x80;
pub const sStatusNormalDelayedTamper : u8 = 0x81;
pub const sStatusAlarmTamper : u8 = 0x82;
pub const sStatusAlarmDelayedTamper : u8 = 0x83;
pub const sStatusMotionTamper : u8 = 0x84;
pub const sStatusNoMotionTamper : u8 = 0x85;*/

//types for Security2
pub const pTypeSecurity2 : u8 = 0x21;
pub const sTypeSec2Classic : u8 = 0x0;

//types for Camera
pub const pTypeCamera : u8 = 0x28;
pub const sTypeNinja : u8 = 0x0;		//X10 Ninja/Robocam
/*pub const camera_sLeft : u8 = 0x0;
pub const camera_sRight : u8 = 0x1;
pub const camera_sUp : u8 = 0x2;
pub const camera_sDown : u8 = 0x3;
pub const camera_sPosition1 : u8 = 0x4;
pub const camera_sProgramPosition1 : u8 = 0x5;
pub const camera_sPosition2 : u8 = 0x6;
pub const camera_sProgramPosition2 : u8 = 0x7;
pub const camera_sPosition3 :u8 = 0x8;
pub const camera_sProgramPosition3 : u8 = 0x9;
pub const camera_sPosition4 : u8 = 0xA;
pub const camera_sProgramPosition4 : u8 = 0xB;
pub const camera_sCenter : u8 = 0xC;
pub const camera_sProgramCenterPosition : u8 = 0xD;
pub const camera_sSweep : u8 = 0xE;
pub const camera_sProgramSweep : u8 = 0xF;*/

//types for Remotes
pub const pTypeRemote : u8 = 0x30;
pub const sTypeATI : u8 = 0x0;		//ATI Remote Wonder
pub const sTypeATIplus : u8 = 0x1;	//ATI Remote Wonder Plus
pub const sTypeMedion : u8 = 0x2;		//Medion Remote
pub const sTypePCremote : u8 = 0x3;	//PC Remote
pub const sTypeATIrw2 : u8 = 0x4;		//ATI Remote Wonder II

//types for Bi-directional Blinds DDxxxx
pub const pTypeDDxxxx : u8 = 0x31;
/*pub const sTypeDDxxxx : u8 = 0x00;
pub const DDxxxx_Up : u8 = 0x0;
pub const DDxxxx_Down : u8 = 0x1;
pub const DDxxxx_Stop : u8 = 0x2;
pub const DDxxxx_P2 : u8 = 0x3;	//Pair
pub const DDxxxx_Percent : u8 = 0x4;
pub const DDxxxx_Angle : u8 = 0x5;
pub const DDxxxx_PercentAngle : u8 = 0x6;
pub const DDxxxx_HoldUp : u8 = 0x7; //not yet used
pub const DDxxxx_HoldStop : u8 = 0x8; //not yet used
pub const DDxxxx_HoldUpDown : u8 = 0x9; //not yet used
pub const DDxxxx_HoldStopUp : u8 = 0xA; //not yet used
pub const DDxxxx_HoldStopDown : u8 = 0xB; //not yet used
*/
//types for Thermostat
pub const pTypeThermostat1 : u8 = 0x40;
pub const sTypeDigimax : u8 = 0x0;		//Digimax
pub const sTypeDigimaxShort : u8 = 0x1;	//Digimax with short format
/*pub const thermostat1_sNoStatus : u8 = 0x0;
pub const thermostat1_sDemand : u8 = 0x1;
pub const thermostat1_sNoDemand : u8 = 0x2;
pub const thermostat1_sInitializing : u8 = 0x3;*/

pub const pTypeThermostat2 : u8 = 0x41;
pub const sTypeHE105 : u8 = 0x0;
pub const sTypeRTS10 : u8 = 0x1;
/*pub const thermostat2_sOff : u8 = 0x0;
pub const thermostat2_sOn : u8 = 0x1;
pub const thermostat2_sProgram : u8 = 0x2;*/

pub const pTypeThermostat3 : u8 = 0x42;
pub const sTypeMertikG6RH4T1 : u8 = 0x0;	//Mertik G6R-H4T1
pub const sTypeMertikG6RH4TB : u8 = 0x1;	//Mertik G6R-H4TB
pub const sTypeMertikG6RH4TD : u8 = 0x2;	//Mertik G6R-H4TD
pub const sTypeMertikG6RH4S : u8 = 0x3;	//Mertik G6R-H4S
/*pub const thermostat3_sOff : u8 = 0x0;
pub const thermostat3_sOn : u8 = 0x1;
pub const thermostat3_sUp : u8 = 0x2;
pub const thermostat3_sDown : u8 = 0x3;
pub const thermostat3_sRunUp : u8 = 0x4;
pub const thermostat3_Off2nd : u8 = 0x4;
pub const thermostat3_sRunDown : u8 = 0x5;
pub const thermostat3_On2nd : u8 = 0x5;
pub const thermostat3_sStop : u8 = 0x6;*/

pub const pTypeThermostat4 : u8 = 0x43;
pub const sTypeMCZ1 : u8 = 0x0;	//MCZ 1 fan model
pub const sTypeMCZ2 : u8 = 0x1;	//MCZ 2 fan model
pub const sTypeMCZ3 : u8 = 0x2;	//MCZ 3 fan model
/*pub const thermostat4_sOff : u8 = 0x0;
pub const thermostat4_sManual : u8 = 0x1;
pub const thermostat4_sAuto : u8 = 0x2;
pub const thermostat4_sEco : u8 = 0x3;*/

pub const pTypeThermostat5 : u8 = 0x44;
/*pub const sTypeGazco : u8 = 0x0;
pub const thermostat5_sOff : u8 = 0x0;
pub const thermostat5_sOn : u8 = 0x1;
pub const thermostat5_sOnHeaterStby : u8 = 0x2;
pub const thermostat5_sOnHeaterLow : u8 = 0x3;
pub const thermostat5_sOnHeaterHigh : u8 = 0x04;*/

//types for Radiator valve
pub const pTypeRadiator1 : u8 = 0x48;
pub const sTypeSmartwares : u8 = 0x0;	//Homewizard smartwares

/*pub const Radiator1_sNight : u8 = 0x0;
pub const Radiator1_sDay : u8 = 0x1;
pub const Radiator1_sSetTemp : u8 = 0x2;*/

//types for BBQ temperature
pub const pTypeBBQ : u8 = 0x4E;
pub const sTypeBBQ1 : u8 = 0x1;  //Maverick ET-732

//types for temperature+rain
pub const pTypeTEMP_RAIN : u8 = 0x4F;
pub const sTypeTR1 : u8 = 0x1;  //WS1200

//types for temperature
pub const pTypeTEMP : u8 = 0x50;
pub const sTypeTEMP1 : u8 = 0x1;  //THR128/138,THC138, Davis
pub const sTypeTEMP2 : u8 = 0x2;  //THC238/268,THN132,THWR288,THRN122,THN122,AW129/131
pub const sTypeTEMP3 : u8 = 0x3;  //THWR800
pub const sTypeTEMP4 : u8 = 0x4;	//RTHN318
pub const sTypeTEMP5 : u8 = 0x5;  //LaCrosse TX3
pub const sTypeTEMP6 : u8 = 0x6;  //TS15C
pub const sTypeTEMP7 : u8 = 0x7;  //Viking 02811,TSS330
pub const sTypeTEMP8 : u8 = 0x8;  //LaCrosse WS2300
pub const sTypeTEMP9 : u8 = 0x9;  //RUBiCSON
pub const sTypeTEMP10 : u8 = 0xA;  //TFA 30.3133
pub const sTypeTEMP11 : u8 = 0xB;  //WT0122

//types for humidity
pub const pTypeHUM : u8 = 0x51;
pub const sTypeHUM1 : u8 = 0x1;  //LaCrosse TX3, Davis
pub const sTypeHUM2 : u8 = 0x2;  //LaCrosse WS2300
//pub const sTypeHUM3 : u8 = 0x03;  //Inovalley S80 plant humidity sensor

//status types for humidity
/*pub const humstat_normal : u8 = 0x0;
pub const humstat_comfort : u8 = 0x1;
pub const humstat_dry : u8 = 0x2;
pub const humstat_wet : u8 = 0x3;*/

//types for temperature+humidity
pub const pTypeTEMP_HUM : u8 = 0x52;
pub const sTypeTH1 : u8 = 0x1;  //THGN122/123,THGN132,THGR122/228/238/268
pub const sTypeTH2 : u8 = 0x2;  //THGR810,THGN800
pub const sTypeTH3 : u8 = 0x3;  //RTGR328
pub const sTypeTH4 : u8 = 0x4;  //THGR328
pub const sTypeTH5 : u8 = 0x5;  //WTGR800
pub const sTypeTH6 : u8 = 0x6;  //THGR918,THGRN228,THGN500
pub const sTypeTH7 : u8 = 0x7;  //TFA TS34C, Cresta
pub const sTypeTH8 : u8 = 0x8;  //WT450H
pub const sTypeTH9 : u8 = 0x9;  //Viking 02035,02038 (02035 has no humidity), TSS320
pub const sTypeTH10 : u8 = 0xA;   //Rubicson
pub const sTypeTH11 : u8 = 0xB;   //EW109
pub const sTypeTH12 : u8 = 0xC;   //Imagintronix
pub const sTypeTH13 : u8 = 0xD;   //Alecto WS1700 and compatibles
pub const sTypeTH14 : u8 = 0xE;   //Alecto

//types for barometric
pub const pTypeBARO : u8 = 0x53;

//types for temperature+humidity+baro
pub const pTypeTEMP_HUM_BARO : u8 = 0x54;
pub const sTypeTHB1 : u8 = 0x1;   //BTHR918,BTHGN129
pub const sTypeTHB2 : u8 = 0x2;   //BTHR918N,BTHR968
/*pub const baroForecastNoInfo : u8 = 0x00;
pub const baroForecastSunny : u8 = 0x01;
pub const baroForecastPartlyCloudy : u8 = 0x02;
pub const baroForecastCloudy : u8 = 0x03;
pub const baroForecastRain : u8 = 0x04;*/

//types for rain
pub const pTypeRAIN : u8 = 0x55;
pub const sTypeRAIN1 : u8 = 0x1;   //RGR126/682/918
pub const sTypeRAIN2 : u8 = 0x2;   //PCR800
pub const sTypeRAIN3 : u8 = 0x3;   //TFA
pub const sTypeRAIN4 : u8 = 0x4;   //UPM
pub const sTypeRAIN5 : u8 = 0x5;   //WS2300
pub const sTypeRAIN6 : u8 = 0x6;   //TX5
pub const sTypeRAIN7 : u8 = 0x7;   //Alecto
pub const sTypeRAIN8 : u8 = 0x8;   //Davis
pub const sTypeRAIN9 : u8 = 0x9;   //TFA 30.3233.01 
pub const sTypeRAIN10 : u8 = 0xA;   //FineOffset WH5360,EcoWitt WH40

//types for wind
pub const pTypeWIND : u8 = 0x56;
pub const sTypeWIND1 : u8 = 0x1;   //WTGR800
pub const sTypeWIND2 : u8 = 0x2;   //WGR800
pub const sTypeWIND3 : u8 = 0x3;   //STR918,WGR918
pub const sTypeWIND4 : u8 = 0x4;   //TFA
pub const sTypeWIND5 : u8 = 0x5;   //UPM, Davis
pub const sTypeWIND6 : u8 = 0x6;   //WS2300
pub const sTypeWIND7 : u8 = 0x7;   //Alecto WS4500

//types for uv
pub const pTypeUV : u8 = 0x57;
pub const sTypeUV1 : u8 = 0x1;   //UVN128,UV138, Davis
pub const sTypeUV2 : u8 = 0x2;   //UVN800
pub const sTypeUV3 : u8 = 0x3;   //TFA

//types for date & time
pub const pTypeDT : u8 = 0x58;
pub const sTypeDT1 : u8 = 0x1;   //RTGR328N

//types for current
pub const pTypeCURRENT : u8 = 0x59;
pub const sTypeELEC1 : u8 = 0x1;   //CM113,Electrisave

//types for energy
pub const pTypeENERGY : u8 = 0x5A;
pub const sTypeELEC2 : u8 = 0x1;   //CM119/160
pub const sTypeELEC3 : u8 = 0x2;   //CM180

//types for current-energy
pub const pTypeCURRENTENERGY : u8 = 0x5B;
pub const sTypeELEC4 : u8 = 0x1;   //CM180i

//types for power
pub const pTypePOWER : u8 = 0x5C;
pub const sTypeELEC5 : u8 = 0x1;   //revolt

//types for weight scales
pub const pTypeWEIGHT : u8 = 0x5D;
pub const sTypeWEIGHT1 : u8 = 0x1;   //BWR102
pub const sTypeWEIGHT2 : u8 = 0x2;   //GR101

//types for gas
pub const pTypeGAS : u8 = 0x5E;

//types for water
pub const pTypeWATER : u8 = 0x5F;

//types for CARTELECTRONIC
pub const pTypeCARTELECTRONIC : u8 = 0x60;
/*pub const sTypeTIC : u8 = 0x1;
pub const sTypeCEencoder : u8 = 0x2;
pub const sTypeLinky : u8 = 0x3;*/

//types for Async port configuration
/*pub const pTypeASYNCPORT : u8 = 0x61;
pub const sTypeASYNCconfig : u8 = 0x01;
pub const asyncdisable : u8 = 0x0;
pub const asyncreceiveP1 : u8 = 0x1;
pub const asyncreceiveTeleinfo : u8 = 0x2;
pub const asyncreceiveRAW : u8 = 0xFE; //not yet implemented
pub const asyncreceiveGetSettings : u8 = 0xFF;
pub const asyncbaud110 : u8 = 0x0;
pub const asyncbaud300 : u8 = 0x1;
pub const asyncbaud600 : u8 = 0x2;
pub const asyncbaud1200 : u8 = 0x3;
pub const asyncbaud2400 : u8 = 0x4;
pub const asyncbaud4800 : u8 = 0x5;
pub const asyncbaud9600 : u8 = 0x6;
pub const asyncbaud14400 : u8 = 0x7;
pub const asyncbaud19200 : u8 = 0x8;
pub const asyncbaud38400 : u8 = 0x9;
pub const asyncbaud57600 : u8 = 0xA;
pub const asyncbaud115200 : u8 = 0xB;
pub const asyncParityNo : u8 = 0x0;
pub const asyncParityOdd : u8 = 0x1;
pub const asyncParityEven : u8 = 0x2;
pub const asyncDatabits7 : u8 = 0x7;
pub const asyncDatabits8 : u8 = 0x8;
pub const asyncStopbits1 : u8 = 0x1;
pub const asyncStopbits2 : u8 = 0x2;
pub const asyncPolarityNormal : u8 = 0x0;
pub const asyncPolarityInvers : u8 = 0x1;

//types for Async data
pub const pTypeASYNCDATA : u8 = 0x62;
pub const sTypeASYNCoverrun : u8 = 0xFF;
pub const sTypeASYNCp1 : u8 = 0x01;
pub const sTypeASYNCteleinfo : u8 = 0x02;
pub const sTypeASYNCraw : u8 = 0x03;*/

//RFXSensor
pub const pTypeRFXSensor : u8 = 0x70;
pub const sTypeRFXSensorTemp : u8 = 0x0;
pub const sTypeRFXSensorAD : u8 = 0x1;
pub const sTypeRFXSensorVolt : u8 = 0x2;
//pub const sTypeRFXSensorMessage : u8 = 0x3;

//RFXMeter
pub const pTypeRFXMeter : u8 = 0x71;
pub const sTypeRFXMeterCount : u8 = 0x0;
/*pub const sTypeRFXMeterInterval : u8 = 0x1;
pub const sTypeRFXMeterCalib : u8 = 0x2;
pub const sTypeRFXMeterAddr : u8 = 0x3;
pub const sTypeRFXMeterCounterReset : u8 = 0x4;
pub const sTypeRFXMeterCounterSet : u8 = 0xB;
pub const sTypeRFXMeterSetInterval : u8 = 0xC;
pub const sTypeRFXMeterSetCalib : u8 = 0xD;
pub const sTypeRFXMeterSetAddr : u8 = 0xE;
pub const sTypeRFXMeterIdent : u8 = 0xF;*/

//FS20
pub const pTypeFS20 : u8 = 0x72;
pub const sTypeFS20 : u8 = 0x0;
pub const sTypeFHT8V : u8 = 0x1;
pub const sTypeFHT80 : u8 = 0x2;
/*pub const fs20_sOff : u8 = 0x0;
pub const fs20_sDimlevel_1 : u8 = 0x1;
pub const fs20_sDimlevel_2 : u8 = 0x2;
pub const fs20_sDimlevel_3 : u8 = 0x3;
pub const fs20_sDimlevel_4 : u8 = 0x4;
pub const fs20_sDimlevel_5 : u8 = 0x5;
pub const fs20_sDimlevel_6 : u8 = 0x6;
pub const fs20_sDimlevel_7 : u8 = 0x7;
pub const fs20_sDimlevel_8 : u8 = 0x8;
pub const fs20_sDimlevel_9 : u8 = 0x9;
pub const fs20_sDimlevel_10 : u8 = 0xA;
pub const fs20_sDimlevel_11 : u8 = 0xB;
pub const fs20_sDimlevel_12 : u8 = 0xC;
pub const fs20_sDimlevel_13 : u8 = 0xD;
pub const fs20_sDimlevel_14 : u8 = 0xE;
pub const fs20_sDimlevel_15 : u8 = 0xF;
pub const fs20_sOn_100 : u8 = 0x10;
pub const fs20_sOn_last_dim : u8 = 0x11;
pub const fs20_sToggle_On_Off : u8 = 0x12;
pub const fs20_sBright : u8 = 0x13;
pub const fs20_sDim : u8 = 0x14;
pub const fs20_sStart_dim_cycle : u8 = 0x15;
pub const fs20_sProgram_timer : u8 = 0x16;
pub const fs20_sRequest_status : u8 = 0x17;
pub const fs20_sOff_for_time_period : u8 = 0x18;
pub const fs20_sOn_100_for_time_period : u8 = 0x19; 
pub const fs20_sOn_last_dim_level_period : u8 = 0x1A; 
pub const fs20_sReset : u8 = 0x1B;*/

//Water level sensor
pub const pTypeLEVELSENSOR : u8 = 0x73;
//pub const sType0 : u8 = 0x0;

//LIGHTNING SENSOR
pub const pTypeLIGHTNING : u8 = 0x74;
//pub const sTypeLIGHTNING1 : u8 = 0x0;   //Ecowitt WH57

//WEATHER STATIONS
pub const pTypeWEATHER : u8 = 0x76;
pub const sTypeWEATHER0 : u8 = 0x0;   //Ecowitt WS90
pub const sTypeWEATHER1 : u8 = 0x1;   //Alecto ACH2010
pub const sTypeWEATHER2 : u8 = 0x2;   //Alecto WS5500

//types for Solar
pub const pTypeSOLAR : u8 = 0x77;
pub const sTypeSOLAR1 : u8 = 0x1;   //Davis

//RAW transit/receive
/*pub const pTypeRAW : u8 = 0x7F;
pub const sTypeRAW1 : u8 = 0x0;
pub const sTypeRAW2 : u8 = 0x1;
pub const sTypeRAW3 : u8 = 0x2;
pub const sTypeRAW4 : u8 = 0x3;
*/

// ColorSwitch
pub const pTypeColorSwitch : u8 = 0xF1;
pub const sTypeColor_RGB_W : u8 = 0x01; // RGB + white, either RGB or white can be lit
pub const sTypeColor_RGB : u8 = 0x02; // RGB
pub const sTypeColor_White : u8 = 0x03; // Monochrome white
pub const sTypeColor_RGB_CW_WW : u8 = 0x04; // RGB + cold white + warm white, either RGB or white can be lit
pub const sTypeColor_LivCol : u8 = 0x05;
pub const sTypeColor_RGB_W_Z : u8 = 0x06; // Like RGBW, but allows combining RGB and white
pub const sTypeColor_RGB_CW_WW_Z : u8 = 0x07; // Like RGBWW, but allows combining RGB and white
pub const sTypeColor_CW_WW : u8 = 0x08; // Cold white + Warm white

pub const pTypeRFY2 : u8 = 0xFE;

pub const DEVICE_TYPES_DESC_DATA : [(u8,(&str,&str));69] = [
	(pTypeInterfaceControl,  ("Interface Control"           , "unknown"     )),   //{ pTypeInterfaceControl,   , "unknown"     },
	(pTypeInterfaceMessage,  ("Interface Message"           , "unknown"     )),   //{ pTypeInterfaceMessage,   , "unknown"     },
	(pTypeRecXmitMessage,    ("Receiver/Transmitter Message", "unknown"     )),   //{ pTypeRecXmitMessage,     , "unknown"     },
	(pTypeUndecoded,         ("Undecoded RF Message"        , "unknown"     )),   //{ pTypeUndecoded,          , "unknown"     },
	(pTypeLighting1,         ("Lighting 1"                  , "lightbulb"   )),   //{ pTypeLighting1,          , "lightbulb"   },
	(pTypeLighting2,         ("Lighting 2"                  , "lightbulb"   )),   //{ pTypeLighting2,          , "lightbulb"   },
	(pTypeLighting3,         ("Lighting 3"                  , "lightbulb"   )),   //{ pTypeLighting3,          , "lightbulb"   },
	(pTypeLighting4,         ("Lighting 4"                  , "lightbulb"   )),   //{ pTypeLighting4,          , "lightbulb"   },
	(pTypeLighting5,         ("Lighting 5"                  , "lightbulb"   )),   //{ pTypeLighting5,          , "lightbulb"   },
	(pTypeLighting6,         ("Lighting 6"                  , "lightbulb"   )),   //{ pTypeLighting6,          , "lightbulb"   },
	(pTypeHomeConfort,       ("Home Confort"                , "lightbulb"   )),   //{ pTypeHomeConfort,        , "lightbulb"   },
	(pTypeColorSwitch,       ("Color Switch"                , "lightbulb"   )),   //{ pTypeColorSwitch,        , "lightbulb"   },
	(pTypeCurtain,           ("Curtain"                     , "blinds"      )),   //{ pTypeCurtain,            , "blinds"      },
	(pTypeBlinds,            ("Blinds"                      , "blinds"      )),   //{ pTypeBlinds,             , "blinds"      },
	(pTypeSecurity1,         ("Security"                    , "security"    )),   //{ pTypeSecurity1,          , "security"    },
	(pTypeSecurity2,         ("Security"                    , "security"    )),   //{ pTypeSecurity2,          , "security"    },
	(pTypeCamera,            ("Camera"                      , "unknown"     )),   //{ pTypeCamera,             , "unknown"     },
	(pTypeRemote,            ("Remote & IR"                 , "unknown"     )),   //{ pTypeRemote,             , "unknown"     },
	(pTypeThermostat1,       ("Thermostat 1"                , "temperature" )),   //{ pTypeThermostat1,        , "temperature" },
	(pTypeThermostat2,       ("Thermostat 2"                , "temperature" )),   //{ pTypeThermostat2,        , "temperature" },
	(pTypeThermostat3,       ("Thermostat 3"                , "temperature" )),   //{ pTypeThermostat3,        , "temperature" },
	(pTypeThermostat4,       ("Thermostat 4"                , "temperature" )),   //{ pTypeThermostat4,        , "temperature" },
	(pTypeThermostat5,       ("Thermostat 5"                , "temperature" )),   //{ pTypeThermostat5,        , "temperature" },
	(pTypeRadiator1,         ("Radiator 1"                  , "temperature" )),   //{ pTypeRadiator1,          , "temperature" },
	(pTypeTEMP,              ("Temp"                        , "temperature" )),   //{ pTypeTEMP,               , "temperature" },
	(pTypeHUM,               ("Humidity"                    , "temperature" )),   //{ pTypeHUM,                , "temperature" },
	(pTypeTEMP_HUM,          ("Temp + Humidity"             , "temperature" )),   //{ pTypeTEMP_HUM,           , "temperature" },
	(pTypeBARO,              ("Barometric"                  , "temperature" )),   //{ pTypeBARO,               , "temperature" },
	(pTypeTEMP_HUM_BARO,     ("Temp + Humidity + Baro"      , "temperature" )),   //{ pTypeTEMP_HUM_BARO,      , "temperature" },
	(pTypeRAIN,              ("Rain"                        , "rain"        )),   //{ pTypeRAIN,               , "rain"        },
	(pTypeWIND,              ("Wind"                        , "wind"        )),   //{ pTypeWIND,               , "wind"        },
	(pTypeUV,                ("UV"                          , "uv"          )),   //{ pTypeUV,                 , "uv"          },
	(pTypeDT,                ("Date/Time"                   , "unknown"     )),   //{ pTypeDT,                 , "unknown"     },
	(pTypeCURRENT,           ("Current"                     , "current"     )),   //{ pTypeCURRENT,            , "current"     },
	(pTypeENERGY,            ("Energy"                      , "current"     )),   //{ pTypeENERGY,             , "current"     },
	(pTypeCURRENTENERGY,     ("Current/Energy"              , "current"     )),   //{ pTypeCURRENTENERGY,      , "current"     },
	(pTypeGAS,               ("Gas"                         , "counter"     )),   //{ pTypeGAS,                , "counter"     },
	(pTypeWATER,             ("Water"                       , "counter"     )),   //{ pTypeWATER,              , "counter"     },
	(pTypeWEIGHT,            ("Weight"                      , "scale"       )),   //{ pTypeWEIGHT,             , "scale"       },
	(pTypeCARTELECTRONIC,    ("CartElectronic Power Sensor" , "unknown"     )),   //{ pTypeCARTELECTRONIC,     , "unknown"     },
	(pTypeRFXSensor,         ("RFXSensor"                   , "unknown"     )),   //{ pTypeRFXSensor,          , "unknown"     },
	(pTypeRFXMeter,          ("RFXMeter"                    , "counter"     )),   //{ pTypeRFXMeter,           , "counter"     },
	(pTypeLEVELSENSOR,       ("Water Level"                 , "counter"     )),   //{ pTypeLEVELSENSOR,        , "counter"     },
	(pTypeLIGHTNING,         ("LIGHTNING"                   , "counter"     )),   //{ pTypeLIGHTNING,          , "counter"     },
	(pTypeP1Power,           ("P1 Smart Meter"              , "counter"     )),   //{ pTypeP1Power,            , "counter"     },
	(pTypeP1Gas,             ("P1 Smart Meter"              , "counter"     )),   //{ pTypeP1Gas,              , "counter"     },
	(pTypeYouLess,           ("YouLess Meter"               , "counter"     )),   //{ pTypeYouLess,            , "counter"     },
	(pTypeFS20,              ("FS20"                        , "lightbulb"   )),   //{ pTypeFS20,               , "lightbulb"   },
	(pTypeRego6XXTemp,       ("Temp"                        , "temperature" )),   //{ pTypeRego6XXTemp,        , "temperature" },
	(pTypeRego6XXValue,      ("Value"                       , "utility"     )),   //{ pTypeRego6XXValue,       , "utility"     },
	(pTypeAirQuality,        ("Air Quality"                 , "air"         )),   //{ pTypeAirQuality,         , "air"         },
	(pTypeUsage,             ("Usage"                       , "current"     )),   //{ pTypeUsage,              , "current"     },
	(pTypeTEMP_BARO,         ("Temp + Baro"                 , "temperature" )),   //{ pTypeTEMP_BARO,          , "temperature" },
	(pTypeLux,               ("Lux"                         , "lux"         )),   //{ pTypeLux,                , "lux"         },
	(pTypeGeneral,           ("General"                     , "General"     )),   //{ pTypeGeneral,            , "General"     },
	(pTypeSetpoint,          ("Setpoint"                    , "setpoint"    )),   //{ pTypeSetpoint,           , "setpoint"    },
	(pTypeTEMP_RAIN,         ("Temp + Rain"                 , "Temp + Rain" )),   //{ pTypeTEMP_RAIN,          , "Temp + Rain" },
	(pTypeChime,             ("Chime"                       , "doorbell"    )),   //{ pTypeChime,              , "doorbell"    },
	(pTypeFan,               ("Fan"                         , "fan"         )),   //{ pTypeFan,                , "fan"         },
	(pTypeBBQ,               ("BBQ Meter"                   , "bbq"         )),   //{ pTypeBBQ,                , "bbq"         },
	(pTypePOWER,             ("Power"                       , "current"     )),   //{ pTypePOWER,              , "current"     },
	(pTypeRFY2,              ("RFY 2"                       , "blinds"      )),   //{ pTypeRFY2,               , "blinds"      },
	(pTypeEvohome,           ("Heating"                     , "evohome"     )),   //{ pTypeEvohome,            , "evohome"     },
	(pTypeEvohomeZone,       ("Heating"                     , "evohome"     )),   //{ pTypeEvohomeZone,        , "evohome"     },
	(pTypeEvohomeWater,      ("Heating"                     , "evohome"     )),   //{ pTypeEvohomeWater,       , "evohome"     },
	(pTypeEvohomeRelay,      ("Heating"                     , "evohome"     )),   //{ pTypeEvohomeRelay,       , "evohome"     },
	(pTypeGeneralSwitch,     ("Light/Switch"                , "lightbulb"   )),   //{ pTypeGeneralSwitch,      , "lightbulb"   },
	(pTypeWEATHER,           ("Weather"                     , "weather"     )),   //{ pTypeWEATHER,            , "weather"     },
	(pTypeSOLAR,             ("Solar"                       , "solar"       )),   //{ pTypeSOLAR,              , "solar"       },
	//{ pTypeRFY, "RFY"   , "blinds" },
	//{ pTypeHunter, "Hunter", "Hunter" },
	//{ pTypeDDxxxx, "Blinds", "blinds" },
];
pub static DEVICE_TYPES_DESC : std::sync::LazyLock<HashMap<u8, (&str, &str)>> = std::sync::LazyLock::new(|| {
	DEVICE_TYPES_DESC_DATA.into()
});

pub const DEVICE_SUBTYPES_DESC_DATA : [((u8, u8), &str);358] = [
	((pTypeTEMP, sTypeTEMP1), "THR128/138, THC138"),
	((pTypeTEMP, sTypeTEMP2), "THC238/268, THN132, THWR288, THRN122, THN122, AW129/131"),
	((pTypeTEMP, sTypeTEMP3), "THWR800"),
	((pTypeTEMP, sTypeTEMP4), "RTHN318"),
	((pTypeTEMP, sTypeTEMP5), "LaCrosse TX3"),
	((pTypeTEMP, sTypeTEMP6), "TS15C"),
	((pTypeTEMP, sTypeTEMP7), "Viking 02811/02813, Proove TSS330"),
	((pTypeTEMP, sTypeTEMP8), "LaCrosse WS2300"),
	((pTypeTEMP, sTypeTEMP9), "RUBiCSON"),
	((pTypeTEMP, sTypeTEMP10), "TFA 30.3133"),
	((pTypeTEMP, sTypeTEMP11), "WT0122 Pool sensor"),
	((pTypeTEMP, sTypeTEMP_SYSTEM), "System"),

	((pTypeHUM, sTypeHUM1), "LaCrosse TX3"),
	((pTypeHUM, sTypeHUM2), "LaCrosse WS2300"),

	((pTypeTEMP_HUM, sTypeTH1), "THGN122/123/132, THGR122/228/238/268"),
	((pTypeTEMP_HUM, sTypeTH2), "THGR810, THGN800"),
	((pTypeTEMP_HUM, sTypeTH3), "RTGR328"),
	((pTypeTEMP_HUM, sTypeTH4), "THGR328"),
	((pTypeTEMP_HUM, sTypeTH5), "WTGR800"),
	((pTypeTEMP_HUM, sTypeTH6), "THGR918, THGRN228, THGN500"),
	((pTypeTEMP_HUM, sTypeTH7), "Cresta, TFA TS34C"),
	((pTypeTEMP_HUM, sTypeTH8), "WT450H"),
	((pTypeTEMP_HUM, sTypeTH9), "Viking 02035, 02038, TSS320"),
	((pTypeTEMP_HUM, sTypeTH10), "Rubicson/IW008T/TX95"),
	((pTypeTEMP_HUM, sTypeTH11), "Oregon EW109"),
	((pTypeTEMP_HUM, sTypeTH12), "Imagintronix"),
	((pTypeTEMP_HUM, sTypeTH13), "Alecto WS1700"),
	((pTypeTEMP_HUM, sTypeTH14), "Alecto"),
	((pTypeTEMP_HUM, sTypeTH_LC_TC), "LaCrosse TX3"),

	((pTypeTEMP_HUM_BARO, sTypeTHB1), "THB1 - BTHR918, BTHGN129"),
	((pTypeTEMP_HUM_BARO, sTypeTHB2), "THB2 - BTHR918N, BTHR968"),
	((pTypeTEMP_HUM_BARO, sTypeTHBFloat), "Weather Station"),

	((pTypeRAIN, sTypeRAIN1), "RGR126/682/918/928"),
	((pTypeRAIN, sTypeRAIN2), "PCR800"),
	((pTypeRAIN, sTypeRAIN3), "TFA"),
	((pTypeRAIN, sTypeRAIN4), "UPM RG700"),
	((pTypeRAIN, sTypeRAIN5), "LaCrosse WS2300"),
	((pTypeRAIN, sTypeRAIN6), "LaCrosse TX5"),
	((pTypeRAIN, sTypeRAIN7), "Alecto"),
	((pTypeRAIN, sTypeRAIN8), "Davis"),
	((pTypeRAIN, sTypeRAIN9), "TFA 30.3233.01"),
	((pTypeRAIN, sTypeRAIN10), "FineOffset WH5360, EcoWitt WH40"),
	((pTypeRAIN, sTypeRAINWU), "WWW"),
	((pTypeRAIN, sTypeRAINByRate), "RainByRate"),

	((pTypeWIND, sTypeWIND1), "WTGR800"),
	((pTypeWIND, sTypeWIND2), "WGR800"),
	((pTypeWIND, sTypeWIND3), "STR918/928, WGR918"),
	((pTypeWIND, sTypeWIND4), "TFA"),
	((pTypeWIND, sTypeWIND5), "UPM WDS500"),
	((pTypeWIND, sTypeWIND6), "LaCrosse WS2300"),
	((pTypeWIND, sTypeWIND7), "Alecto WS4500"),
	((pTypeWIND, sTypeWINDNoTemp), "Weather Station"),
	((pTypeWIND, sTypeWINDNoTempNoChill), "Wind"),

	((pTypeUV, sTypeUV1), "UVN128,UV138"),
	((pTypeUV, sTypeUV2), "UVN800"),
	((pTypeUV, sTypeUV3), "TFA"),

	((pTypeWEATHER, sTypeWEATHER0), "Ecowitt WS90"),
	((pTypeWEATHER, sTypeWEATHER1), "Alecto ACH2010"),
	((pTypeWEATHER, sTypeWEATHER2), "Alecto WS5500"),

	((pTypeSOLAR, sTypeSOLAR1), "Davis"),

	((pTypeHunter, sTypeHunterfan), "Hunter Fan"),

	((pTypeLighting1, sTypeX10), "X10"),
	((pTypeLighting1, sTypeARC), "ARC"),
	((pTypeLighting1, sTypeAB400D), "ELRO AB400"),
	((pTypeLighting1, sTypeWaveman), "Waveman"),
	((pTypeLighting1, sTypeEMW200), "EMW200"),
	((pTypeLighting1, sTypeIMPULS), "Impuls"),
	((pTypeLighting1, sTypeRisingSun), "RisingSun"),
	((pTypeLighting1, sTypePhilips), "Philips"),
	((pTypeLighting1, sTypeEnergenie), "Energenie"),
	((pTypeLighting1, sTypeEnergenie5), "Energenie 5-gang"),
	((pTypeLighting1, sTypeGDR2), "COCO GDR2"),
	((pTypeLighting1, sTypeHQ), "HQ COCO-20"),
	((pTypeLighting1, sTypeOase), "Oase Inscenio"),

	((pTypeLighting2, sTypeAC), "AC"),
	((pTypeLighting2, sTypeHEU), "HomeEasy EU"),
	((pTypeLighting2, sTypeANSLUT), "Anslut"),

	((pTypeLighting3, sTypeKoppla), "Ikea Koppla"),

	((pTypeLighting4, sTypePT2262), "PT2262"),

	((pTypeLighting5, sTypeLightwaveRF), "LightwaveRF"),
	((pTypeLighting5, sTypeEMW100), "EMW100"),
	((pTypeLighting5, sTypeBBSB), "BBSB new"),
	((pTypeLighting5, sTypeMDREMOTE), "MDRemote"),
	((pTypeLighting5, sTypeRSL), "Conrad RSL"),
	((pTypeLighting5, sTypeLivolo), "Livolo"),
	((pTypeLighting5, sTypeTRC02), "TRC02 (RGB)"),
	((pTypeLighting5, sTypeTRC02_2), "TRC02_2 (RGB)"),
	((pTypeLighting5, sTypeAoke), "Aoke"),
	((pTypeLighting5, sTypeEurodomest), "Eurodomest"),
	((pTypeLighting5, sTypeLivolo1to10), "Livolo 1 to 10"),
	((pTypeLighting5, sTypeRGB432W), "RGB432W"),
	((pTypeLighting5, sTypeMDREMOTE107), "MDRemote 107"),
	((pTypeLighting5, sTypeLegrandCAD), "Legrand CAD"),
	((pTypeLighting5, sTypeAvantek), "Avantek"),
	((pTypeLighting5, sTypeIT), "Intertek,FA500,PROmax"),
	((pTypeLighting5, sTypeMDREMOTE108), "MDRemote 108"),
	((pTypeLighting5, sTypeKangtai), "Kangtai / Cotech"),

	((pTypeLighting6, sTypeBlyss), "Blyss"),

	((pTypeHomeConfort, sTypeHomeConfortTEL010), "TEL-010"),

	((pTypeCurtain, sTypeHarrison), "Harrison"),

	((pTypeBlinds, sTypeBlindsT0), "RollerTrol/Hasta"),
	((pTypeBlinds, sTypeBlindsT1), "Hasta old"),
	((pTypeBlinds, sTypeBlindsT2), "A-OK RF01"),
	((pTypeBlinds, sTypeBlindsT3), "A-OK AC114"),
	((pTypeBlinds, sTypeBlindsT4), "RAEX"),
	((pTypeBlinds, sTypeBlindsT5), "Media Mount"),
	((pTypeBlinds, sTypeBlindsT6), "DC106"),
	((pTypeBlinds, sTypeBlindsT7), "Forest"),
	((pTypeBlinds, sTypeBlindsT8), "Chamberlain"),
	((pTypeBlinds, sTypeBlindsT9), "Sunpery"),
	((pTypeBlinds, sTypeBlindsT10), "Dolat DLM-1"),
	((pTypeBlinds, sTypeBlindsT11), "ASP"),
	((pTypeBlinds, sTypeBlindsT12), "Confexx"),
	((pTypeBlinds, sTypeBlindsT13), "Screenline"),
	((pTypeBlinds, sTypeBlindsT14), "Hualite"),
	((pTypeBlinds, sTypeBlindsT15), "RFU"),
	((pTypeBlinds, sTypeBlindsT16), "Zemismart"),
	((pTypeBlinds, sTypeBlindsT17), "Gaposa"),
	((pTypeBlinds, sTypeBlindsT18), "Cherubini"),

	((pTypeSecurity1, sTypeSecX10), "X10 security"),
	((pTypeSecurity1, sTypeSecX10M), "X10 security motion"),
	((pTypeSecurity1, sTypeSecX10R), "X10 security remote"),
	((pTypeSecurity1, sTypeKD101), "KD101 smoke detector"),
	((pTypeSecurity1, sTypePowercodeSensor), "Visonic sensor - primary contact"),
	((pTypeSecurity1, sTypePowercodeMotion), "Visonic motion"),
	((pTypeSecurity1, sTypeCodesecure), "Visonic CodeSecure"),
	((pTypeSecurity1, sTypePowercodeAux), "Visonic sensor - auxiliary contact"),
	((pTypeSecurity1, sTypeMeiantech), "Meiantech/Atlantic/Aidebao"),
	((pTypeSecurity1, sTypeSA30), "Alecto SA30 smoke detector"),
	((pTypeSecurity1, sTypeRM174RF), "Smartwares RM174RF smoke detector"),
	((pTypeSecurity1, sTypeDomoticzSecurity), "Security Panel"),

	((pTypeSecurity2, sTypeSec2Classic), "KeeLoq"),

	((pTypeCamera, sTypeNinja), "Meiantech"),

	((pTypeRemote, sTypeATI), "ATI Remote Wonder"),
	((pTypeRemote, sTypeATIplus), "ATI Remote Wonder Plus"),
	((pTypeRemote, sTypeMedion), "Medion Remote"),
	((pTypeRemote, sTypePCremote), "PC Remote"),
	((pTypeRemote, sTypeATIrw2), "ATI Remote Wonder II"),

	((pTypeThermostat1, sTypeDigimax), "Digimax"),
	((pTypeThermostat1, sTypeDigimaxShort), "Digimax short"),

	((pTypeThermostat2, sTypeHE105), "HE105"),
	((pTypeThermostat2, sTypeRTS10), "RTS10"),

	((pTypeThermostat3, sTypeMertikG6RH4T1), "Mertik G6R-H4T1"),
	((pTypeThermostat3, sTypeMertikG6RH4TB), "Mertik G6R-H4TB"),
	((pTypeThermostat3, sTypeMertikG6RH4TD), "Mertik G6R-H4TD"),
	((pTypeThermostat3, sTypeMertikG6RH4S), "Mertik G6R-H4S"),

	((pTypeThermostat4, sTypeMCZ1), "MCZ 1 fan model"),
	((pTypeThermostat4, sTypeMCZ2), "MCZ 2 fan model"),
	((pTypeThermostat4, sTypeMCZ3), "MCZ 3 fan model"),

	((pTypeRadiator1, sTypeSmartwares), "Smartwares"),
	((pTypeRadiator1, sTypeSmartwaresSwitchRadiator), "Smartwares Mode"),

	((pTypeDT, sTypeDT1), "RTGR328N"),

	((pTypeCURRENT, sTypeELEC1), "CM113, Electrisave"),

	((pTypeENERGY, sTypeELEC2), "CM119 / CM160"),
	((pTypeENERGY, sTypeELEC3), "CM180"),

	((pTypeCURRENTENERGY, sTypeELEC4), "CM180i"),

	((pTypeWEIGHT, sTypeWEIGHT1), "BWR102"),
	((pTypeWEIGHT, sTypeWEIGHT2), "GR101"),

	((pTypeRFXSensor, sTypeRFXSensorTemp), "Temperature"),
	((pTypeRFXSensor, sTypeRFXSensorAD), "A/D"),
	((pTypeRFXSensor, sTypeRFXSensorVolt), "Voltage"),

	((pTypeRFXMeter, sTypeRFXMeterCount), "RFXMeter counter"),

	((pTypeP1Power, sTypeP1Power), "Energy"),
	((pTypeP1Gas, sTypeP1Gas), "Gas"),

	((pTypeYouLess, sTypeYouLess), "YouLess counter"),

	((pTypeRego6XXTemp, sTypeRego6XXTemp), "Rego 6XX"),
	((pTypeRego6XXValue, sTypeRego6XXStatus), "Rego 6XX"),
	((pTypeRego6XXValue, sTypeRego6XXCounter), "Rego 6XX"),

	((pTypeAirQuality, sTypeVoc), "Voc"),

	((pTypeUsage, sTypeElectric), "Electric"),

	((pTypeTEMP_BARO, sTypeBMP085), "BMP085 I2C"),

	((pTypeLux, sTypeLux), "Lux"),

	((pTypeGeneral, sTypeVisibility), "Visibility"),
	((pTypeGeneral, sTypeSolarRadiation), "Solar Radiation"),
	((pTypeGeneral, sTypeSoilMoisture), "Soil Moisture"),
	((pTypeGeneral, sTypeLeafWetness), "Leaf Wetness"),
	((pTypeGeneral, sTypeSystemTemp), "System temperature"),
	((pTypeGeneral, sTypePercentage), "Percentage"),
	((pTypeGeneral, sTypeFan), "Fan"),
	((pTypeGeneral, sTypeVoltage), "Voltage"),
	((pTypeGeneral, sTypeCurrent), "Current"),
	((pTypeGeneral, sTypePressure), "Pressure"),
	((pTypeGeneral, sTypeBaro), "Barometer"),
	((pTypeGeneral, sTypeSetPoint), "Setpoint"),
	((pTypeGeneral, sTypeTemperature), "Temperature"),
	((pTypeGeneral, sTypeTextStatus), "Text"),

	((pTypeGeneral, sTypeZWaveThermostatMode), "Thermostat Mode"),
	((pTypeGeneral, sTypeZWaveThermostatFanMode), "Thermostat Fan Mode"),
	((pTypeGeneral, sTypeZWaveThermostatOperatingState), "Thermostat Operating State"),
	((pTypeGeneral, sTypeZWaveAlarm), "Alarm"),

	((pTypeGeneral, sTypeAlert), "Alert"),
	((pTypeGeneral, sTypeSoundLevel), "Sound Level"),
	((pTypeGeneral, sTypeUV), "UV"),
	((pTypeGeneral, sTypeDistance), "Distance"),
	((pTypeGeneral, sTypeCounterIncremental), "Counter Incremental"),
	((pTypeGeneral, sTypeKwh), "kWh"),
	((pTypeGeneral, sTypeWaterflow), "Waterflow"),
	((pTypeGeneral, sTypeCustom), "Custom Sensor"),
	((pTypeGeneral, sTypeManagedCounter), "Managed Counter"),

	((pTypeSetpoint, sTypeSetpoint), "SetPoint"),
	((pTypeSetpoint, sTypeThermTemperature), "Temperature"),

	((pTypeChime, sTypeByronSX), "Byron SX"),
	((pTypeChime, sTypeByronMP001), "Byron MP001"),
	((pTypeChime, sTypeSelectPlus), "SelectPlus"),
	((pTypeChime, sTypeByronBY), "Byron BY"),
	((pTypeChime, sTypeEnvivo), "Envivo"),
	((pTypeChime, sTypeAlfawise), "Alfawise"),

	((pTypeFan, sTypeSiemensSF01), "Siemens SF01"),
	((pTypeFan, sTypeItho), "Itho CVE RFT"),
	((pTypeFan, sTypeLucciAir), "Lucci Air"),
	((pTypeFan, sTypeSeavTXS4), "SEAV TXS4"),
	((pTypeFan, sTypeWestinghouse), "Westinghouse"),
	((pTypeFan, sTypeLucciAirDC), "Lucci Air DC"),
	((pTypeFan, sTypeCasafan), "Casafan"),
	((pTypeFan, sTypeFT1211R), "FT1211R"),
	((pTypeFan, sTypeFalmec), "Falmec"),
	((pTypeFan, sTypeLucciAirDCII), "Lucci Air DC II"),
	((pTypeFan, sTypeIthoECO), "Itho ECO"),
	((pTypeFan, sTypeNovy), "Novy"),
	((pTypeFan, sTypeOrcon), "Orcon"),
	((pTypeFan, sTypeIthoHRU400), "Itho HRU400"),

	((pTypeTEMP_RAIN, sTypeTR1), "Alecto WS1200"),

	((pTypeBBQ, sTypeBBQ1), "Maverick ET-732"),

	((pTypePOWER, sTypeELEC5), "Revolt"),

	((pTypeColorSwitch, sTypeColor_RGB_W), "RGBW"),
	((pTypeColorSwitch, sTypeColor_RGB), "RGB"),
	((pTypeColorSwitch, sTypeColor_White), "White"),
	((pTypeColorSwitch, sTypeColor_RGB_CW_WW), "RGBWW"),
	((pTypeColorSwitch, sTypeColor_LivCol), "RGB"),
	((pTypeColorSwitch, sTypeColor_RGB_W_Z), "RGBWZ"),
	((pTypeColorSwitch, sTypeColor_RGB_CW_WW_Z), "RGBWWZ"),
	((pTypeColorSwitch, sTypeColor_CW_WW), "WW"),

	((pTypeRFY, sTypeRFY), "RFY"),
	((pTypeRFY, sTypeRFY2), "RFY2"),
	((pTypeRFY, sTypeRFYext), "RFY-Ext"),
	((pTypeRFY, sTypeASA), "ASA"),

	((pTypeEvohome, sTypeEvohome), "Evohome"),
	((pTypeEvohomeZone, sTypeEvohomeZone), "Zone"),
	((pTypeEvohomeWater, sTypeEvohomeWater), "Hot Water"),
	((pTypeEvohomeRelay, sTypeEvohomeRelay), "Relay"),

	((pTypeFS20, sTypeFS20), "FS20"),
	((pTypeFS20, sTypeFHT8V), "FHT 8V valve"),
	((pTypeFS20, sTypeFHT80), "FHT80 door/window sensor"),

	((pTypeGeneralSwitch, sSwitchTypeX10), "X10"),
	((pTypeGeneralSwitch, sSwitchTypeARC), "ARC"),
	((pTypeGeneralSwitch, sSwitchTypeAB400D), "ELRO AB400"),
	((pTypeGeneralSwitch, sSwitchTypeWaveman), "Waveman"),
	((pTypeGeneralSwitch, sSwitchTypeEMW200), "EMW200"),
	((pTypeGeneralSwitch, sSwitchTypeIMPULS), "Impuls"),
	((pTypeGeneralSwitch, sSwitchTypeRisingSun), "RisingSun"),
	((pTypeGeneralSwitch, sSwitchTypePhilips), "Philips"),
	((pTypeGeneralSwitch, sSwitchTypeEnergenie), "Energenie"),
	((pTypeGeneralSwitch, sSwitchTypeEnergenie5), "Energenie 5-gang"),
	((pTypeGeneralSwitch, sSwitchTypeGDR2), "COCO GDR2"),
	((pTypeGeneralSwitch, sSwitchTypeAC), "AC"),
	((pTypeGeneralSwitch, sSwitchTypeHEU), "HomeEasy EU"),
	((pTypeGeneralSwitch, sSwitchTypeANSLUT), "Anslut"),
	((pTypeGeneralSwitch, sSwitchTypeKoppla), "Ikea Koppla"),
	((pTypeGeneralSwitch, sSwitchTypePT2262), "PT2262"),
	((pTypeGeneralSwitch, sSwitchTypeLightwaveRF), "LightwaveRF"),
	((pTypeGeneralSwitch, sSwitchTypeEMW100), "EMW100"),
	((pTypeGeneralSwitch, sSwitchTypeBBSB), "BBSB new"),
	((pTypeGeneralSwitch, sSwitchTypeMDREMOTE), "MDRemote"),
	((pTypeGeneralSwitch, sSwitchTypeRSL), "Conrad RSL"),
	((pTypeGeneralSwitch, sSwitchTypeLivolo), "Livolo"),
	((pTypeGeneralSwitch, sSwitchTypeTRC02), "TRC02 (RGB)"),
	((pTypeGeneralSwitch, sSwitchTypeTRC02_2), "TRC02_2 (RGB)"),
	((pTypeGeneralSwitch, sSwitchTypeAoke), "Aoke"),
	((pTypeGeneralSwitch, sSwitchTypeEurodomest), "Eurodomest"),
	((pTypeGeneralSwitch, sSwitchTypeLivoloAppliance), "Livolo Appliance"),
	((pTypeGeneralSwitch, sSwitchTypeBlyss), "Blyss"),
	((pTypeGeneralSwitch, sSwitchTypeByronSX), "ByronSX"),
	((pTypeGeneralSwitch, sSwitchTypeByronMP001), "Byron MP001"),
	((pTypeGeneralSwitch, sSwitchTypeSelectPlus), "SelectPlus"),
	((pTypeGeneralSwitch, sSwitchTypeSelectPlus3), "SelectPlus3"),
	((pTypeGeneralSwitch, sSwitchTypeFA20), "FA20RF"),
	((pTypeGeneralSwitch, sSwitchTypeChuango), "Chuango"),
	((pTypeGeneralSwitch, sSwitchTypePlieger), "Plieger"),
	((pTypeGeneralSwitch, sSwitchTypeSilvercrest), "SilverCrest"),
	((pTypeGeneralSwitch, sSwitchTypeMertik), "Mertik"),
	((pTypeGeneralSwitch, sSwitchTypeHomeConfort), "HomeConfort"),
	((pTypeGeneralSwitch, sSwitchTypePowerfix), "Powerfix"),
	((pTypeGeneralSwitch, sSwitchTypeTriState), "TriState"),
	((pTypeGeneralSwitch, sSwitchTypeDeltronic), "Deltronic"),
	((pTypeGeneralSwitch, sSwitchTypeFA500), "FA500"),
	((pTypeGeneralSwitch, sSwitchTypeHT12E), "HT12E"),
	((pTypeGeneralSwitch, sSwitchTypeEV1527), "EV1527"),
	((pTypeGeneralSwitch, sSwitchTypeElmes), "Elmes"),
	((pTypeGeneralSwitch, sSwitchTypeAster), "Aster"),
	((pTypeGeneralSwitch, sSwitchTypeSartano), "Sartano"),
	((pTypeGeneralSwitch, sSwitchTypeEurope), "Europe"),
	((pTypeGeneralSwitch, sSwitchTypeAvidsen), "Avidsen"),
	((pTypeGeneralSwitch, sSwitchTypeBofu), "BofuMotor"),
	((pTypeGeneralSwitch, sSwitchTypeBrel), "BrelMotor"),
	((pTypeGeneralSwitch, sSwitchTypeRTS), "RTS"),
	((pTypeGeneralSwitch, sSwitchTypeElroDB), "ElroDB"),
	((pTypeGeneralSwitch, sSwitchTypeDooya), "Dooya"),
	((pTypeGeneralSwitch, sSwitchTypeUnitec), "Unitec"),
	((pTypeGeneralSwitch, sSwitchTypeSelector), "Selector Switch"),
	((pTypeGeneralSwitch, sSwitchTypeMaclean), "Maclean"),
	((pTypeGeneralSwitch, sSwitchTypeR546), "R546"),
	((pTypeGeneralSwitch, sSwitchTypeDiya), "Diya"),
	((pTypeGeneralSwitch, sSwitchTypeX10secu), "X10Secure"),
	((pTypeGeneralSwitch, sSwitchTypeAtlantic), "Atlantic"),
	((pTypeGeneralSwitch, sSwitchTypeSilvercrestDB), "SilvercrestDB"),
	((pTypeGeneralSwitch, sSwitchTypeMedionDB), "MedionDB"),
	((pTypeGeneralSwitch, sSwitchTypeVMC), "VMC"),
	((pTypeGeneralSwitch, sSwitchTypeKeeloq), "Keeloq"),
	((pTypeGeneralSwitch, sSwitchCustomSwitch), "CustomSwitch"),
	((pTypeGeneralSwitch, sSwitchGeneralSwitch), "Switch"),
	((pTypeGeneralSwitch, sSwitchTypeKoch), "Koch"),
	((pTypeGeneralSwitch, sSwitchTypeKingpin), "Kingpin"),
	((pTypeGeneralSwitch, sSwitchTypeFunkbus), "Funkbus"),
	((pTypeGeneralSwitch, sSwitchTypeNice), "Nice"),
	((pTypeGeneralSwitch, sSwitchTypeForest), "Forest"),
	((pTypeGeneralSwitch, sSwitchBlindsT1), "Legrand MyHome Blind Bus"),
	((pTypeGeneralSwitch, sSwitchLightT1), "Legrand MyHome Light Bus"),
	((pTypeGeneralSwitch, sSwitchAuxiliaryT1), "Legrand MyHome Auxiliary Bus"),
	((pTypeGeneralSwitch, sSwitchContactT1), "Legrand MyHome Contact"),
	((pTypeGeneralSwitch, sSwitchMC145026), "MC145026"),
	((pTypeGeneralSwitch, sSwitchLobeco), "Lobeco"),
	((pTypeGeneralSwitch, sSwitchFriedland), "Friedland"),
	((pTypeGeneralSwitch, sSwitchBFT), "BFT"),
	((pTypeGeneralSwitch, sSwitchNovatys), "Novatys"),
	((pTypeGeneralSwitch, sSwitchHalemeier), "Halemeier"),
	((pTypeGeneralSwitch, sSwitchGaposa), "Gaposa"),
	((pTypeGeneralSwitch, sSwitchMiLightv1), "MiLightv1"),
	((pTypeGeneralSwitch, sSwitchMiLightv2), "MiLightv2"),
	((pTypeGeneralSwitch, sSwitchHT6P20), "HT6P20"),
	((pTypeGeneralSwitch, sSwitchTypeDoitrand), "Doitrand"),
	((pTypeGeneralSwitch, sSwitchTypeWarema), "Warema"),
	((pTypeGeneralSwitch, sSwitchTypeAnsluta), "Ansluta"),
	((pTypeGeneralSwitch, sSwitchTypeLivcol), "Livcol"),
	((pTypeGeneralSwitch, sSwitchTypeBosch), "Bosch"),
	((pTypeGeneralSwitch, sSwitchTypeNingbo), "Ningbo"),
	((pTypeGeneralSwitch, sSwitchTypeDitec), "Ditec"),
	((pTypeGeneralSwitch, sSwitchTypeSteffen), "Steffen"),
	((pTypeGeneralSwitch, sSwitchTypeAlectoSA), "AlectoSA"),
	((pTypeGeneralSwitch, sSwitchTypeGPIOset), "GPIOset"),
	((pTypeGeneralSwitch, sSwitchTypeKonigSec), "KonigSec"),
	((pTypeGeneralSwitch, sSwitchTypeRM174RF), "RM174RF"),
	((pTypeGeneralSwitch, sSwitchTypeLiwin), "Liwin"),
	((pTypeGeneralSwitch, sSwitchBlindsT2), "Legrand MyHome Blind Zigbee"),
	((pTypeGeneralSwitch, sSwitchLightT2), "Legrand MyHome Light Zigbee"),
	((pTypeGeneralSwitch, sSwitchTypeYW_Secu), "YW_Secu"),
	((pTypeGeneralSwitch, sSwitchTypeMertik_GV60), "Mertik_GV60"),
	((pTypeGeneralSwitch, sSwitchTypeNingbo64), "Ningbo64"),
	((pTypeGeneralSwitch, sSwitchTypeX2D), "X2D"),
	((pTypeGeneralSwitch, sSwitchTypeHRCMotor), "HRCMotor"),
	((pTypeGeneralSwitch, sSwitchTypeVelleman), "Velleman"),
	((pTypeGeneralSwitch, sSwitchTypeRFCustom), "RFCustom"),
	((pTypeGeneralSwitch, sSwitchTypeYW_Sensor), "YW_Sensor"),
	((pTypeGeneralSwitch, sSwitchTypeLegrandcad), "LEGRANDCAD"),
	((pTypeGeneralSwitch, sSwitchTypeSysfsGpio), "SysfsGpio"),
	((pTypeGeneralSwitch, sSwitchTypeHager), "Hager"),
	((pTypeGeneralSwitch, sSwitchTypeFaber), "Faber"),
	((pTypeGeneralSwitch, sSwitchTypeDrayton), "Drayton"),
	((pTypeGeneralSwitch, sSwitchTypeV2Phoenix), "V2Phoenix"),
];

pub static DEVICE_SUBTYPES_DESC : std::sync::LazyLock<HashMap<(u8,u8), &str>> = std::sync::LazyLock::new(|| {
	DEVICE_SUBTYPES_DESC_DATA.into()
});
pub const SWITCH_TYPES : [&str;22] =
	[
		"On/Off",
		"Doorbell",
		"Contact",
		"Blinds",
		"X10 Siren",
		"Smoke Detector",
		"",
		"Dimmer",
		"Motion Sensor",
		"Push On Button",
		"Push Off Button",
		"Door Contact",
		"Dusk Sensor",
		"Blinds Percentage",
		"Venetian Blinds US",
		"Venetian Blinds EU",
		"",
		"Media Player",
		"Selector",
		"Door Lock",
		"Door Lock Inverted",
		"Blinds + Stop"
	];

pub const METER_TYPES : [&str;6] =
[
	"Energy",
	"Gas",
	"Water",
	"Custom",
	"Energy Generated",
	"Time"
];