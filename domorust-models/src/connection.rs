
use chrono::{DateTime, Utc};
use serde::Serialize;
//use tokio::net::TcpStream;


pub trait IConnection {
	fn connect(&mut self, timeout:u32);
	fn send(&mut self, message:String /*, Byte, ByteArray, Object*/, delay: f32);
	fn listen(&mut self);
	fn disconnect(&mut self);
	fn connecting(&self) -> bool;
	fn connected(&self) -> bool;
	fn last_seen(&self) -> DateTime<Utc>;
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SerialConnection
{
	pub name: String,
	//pub target,
	pub address: String,
	pub port: u32,
	pub baud: u32,
	pub timeout: i32,
	//pub last_seen,
	//pub pPlugin: &Hardware,
	//pub Parent: &Connection
}

#[derive(Debug, Default, Serialize)]
#[allow(unused)]
pub struct TCPConnection
{
	pub name: String,
	//pub target,
	pub address: String,
	pub port: u32,
	pub baud: u32,
	pub timeout: i32,
	//pub last_seen,
	//pub pPlugin: &Hardware,
	//pub Parent: &Connection
	//#[serde(skip)]
	//stream:Option<TcpStream>
}
#[allow(unused)]
impl IConnection for TCPConnection {
	fn connect(&mut self, timeout:u32) {
		/*tokio::spawn(async {
			let socket = match tokio::time::timeout(
				Duration::from_millis(timeout as u64),
				tokio::net::TcpStream::connect(self.address+":"+self.port.to_string().as_str())
			).await
			{
				Ok(ok) => ok,
				Err(e) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, format!("timeout while connecting to server : {}", e)))
			}.expect("Error while connecting to server");
		});*/
	}

	fn send(&mut self, message:String /*, Byte, ByteArray, Object*/, delay: f32) {
		todo!()
	}

	fn listen(&mut self) {
		/*let listener = TcpListener::bind(self.address+":"+self.port.to_string().as_str()).await?;

    	// accept connections and process them serially
		for stream in listener.incoming() {
			handle_client(stream?);
		}
		Ok(())*/
	}

	fn disconnect(&mut self) {
		todo!()
	}

	fn connecting(&self) -> bool {
		todo!()
	}

	fn connected(&self) -> bool {
		todo!()
	}

	fn last_seen(&self) -> DateTime<Utc> {
		todo!()
	}
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct UDPConnection
{
	pub name: String,
	//pub target,
	pub address: String,
	pub port: u32,
	pub baud: u32,
	pub timeout: i32,
	//pub last_seen,
	//pub pPlugin: &Hardware,
	//pub Parent: &Connection
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct HttpConnection
{
	pub name: String,
	//pub target,
	pub address: String,
	pub port: u32,
	pub baud: u32,
	pub timeout: i32,
	//pub last_seen,
	//pub pPlugin: &Hardware,
	//pub Parent: &Connection
}