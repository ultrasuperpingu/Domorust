pub fn is_neg_or_zero(v:&i16) -> bool {*v <= 0}
//pub fn neg_f32(v:&f32) -> bool {*v < 0 as f32}
pub fn is_nan_f32(v:&f32) -> bool {f32::is_nan(*v)}
pub fn is_u8_max(v:&u8) -> bool {*v == u8::MAX}
//pub fn is_u16_max(v:&u16) -> bool {*v == u16::MAX}
//pub fn is_u32_max(v:&u32) -> bool {*v == u32::MAX}
//pub fn is_usize_max(v:&usize) -> bool {*v == usize::MAX}
pub mod string {
	use std::fmt::Display;
	//use std::str::FromStr;
	use serde::Serializer;
	//use serde::{de, Deserialize, Deserializer};

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
		where T: Display,
				S: Serializer
	{
		serializer.collect_str(value)
	}

	/*pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
		where T: FromStr,
				T::Err: Display,
				D: Deserializer<'de>
	{
		String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
	}*/
}
pub mod base64_decoded {
	use std::fmt::Display;
	//use std::str::FromStr;
	use serde::Serializer;
	//use serde::{de, Deserialize, Deserializer};

	use super::base64;

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
		where T: Display,
				S: Serializer
	{
		let str=value.to_string();
		let dec=base64::decode(str).map_err(serde::ser::Error::custom)?;
		let str=dec.as_slice();
		serializer.collect_str(&std::str::from_utf8(str).map_err(serde::ser::Error::custom)?)
	}

	/*pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
		where T: FromStr,
				T::Err: Display,
				D: Deserializer<'de>
	{
		let str=String::deserialize(deserializer)?;
		let enc=base64::encode(str);
		
		enc.parse().map_err(de::Error::custom)
	}*/
}

pub mod base64_encoded {
	use std::fmt::Display;
	//use std::str::FromStr;
	use serde::Serializer;
	//use serde::{de, Deserialize, Deserializer};

	use super::base64;

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
		where T: Display,
				S: Serializer
	{
		let str=value.to_string();
		let enc=base64::encode(str);
		serializer.collect_str(&enc)
	}

	/*pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
		where T: FromStr,
				T::Err: Display,
				D: Deserializer<'de>
	{
		let str=String::deserialize(deserializer)?;
		let dec=base64::decode(str).map_err(de::Error::custom)?;
		let str=dec.as_slice();
		let str = std::str::from_utf8(str).map_err(de::Error::custom)?;
		
		str.parse().map_err(de::Error::custom)
	}*/
}

pub mod base64 {
	use base64::Engine;

	pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
		base64::prelude::BASE64_STANDARD.decode(input)
	}

	pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
		base64::prelude::BASE64_STANDARD.encode(input)
	}
}
pub mod string_to_f32 {
	use std::fmt::Display;
	//use std::str::FromStr;

	use serde::Serializer;
	//use serde::{de, Deserialize, Deserializer};

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
		where T: Display,
				S: Serializer
	{
		let str=value.to_string();
		serializer.collect_str(&str)
	}

	/*pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
		where T: FromStr,
				T::Err: Display,
				D: Deserializer<'de>
	{
		let str=String::deserialize(deserializer)?;
		str.parse().map_err(de::Error::custom)
	}*/
}