
use std::sync::{Arc, LazyLock};
use std::collections::HashMap;
use async_session::async_trait;
use chrono::{NaiveDateTime, Utc};
use rusqlite::Connection;

use tokio::sync::RwLock;
use warp_sessions::{CookieOptions, SameSiteCookieOption, Session, SessionStore};
use async_session::Result;

use domorust_models::users::User;

use crate::db;
pub static COOKIE : LazyLock<Option<CookieOptions>>=LazyLock::new(|| Some(CookieOptions {
	cookie_name: "sid",
	cookie_value: None,
	max_age: None,
	domain: None,
	path: Some(String::from("/domorust-api")),
	secure: false,
	http_only: false,
	same_site: Some(SameSiteCookieOption::Strict),
}));
#[derive(Debug, Clone, Default)]
pub struct MySessionStore {
	inner: Arc<RwLock<HashMap<String, Session>>>,
}
impl MySessionStore {
	pub fn new() -> Self {
		Self::default()
	}
}
#[async_trait]
impl SessionStore for MySessionStore {
	async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
		let id = Session::id_from_cookie_value(&cookie_value)?;
		let memsession=self
			.inner
			.read()
			.await
			.get(&id)
			.cloned()
			.and_then(Session::validate);
		if memsession.is_some() {
			return Ok(memsession);
		}
		println!("loading session by id `{}` (db)", id);
		let connection = Connection::open("domorust.db")?;
		let session = connection.query_row("SELECT * FROM UserSessions WHERE SessionID=?1", [id.clone()], |row|
			{
				let mut s=Session::new();
				s.set_cookie_value(cookie_value);
				//TODO: maybe change session id in db instead of setting session id to the one in db?
				s.set_id(id);
				s.insert("Username", row.get::<&str,String>("Username")?).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, e.into()))?;
				s.insert("AuthToken", row.get::<&str,String>("AuthToken")?).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, e.into()))?;
				s.insert("RemoteHost", row.get::<&str,String>("RemoteHost")?).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, e.into()))?;
				let expire=row.get::<&str,NaiveDateTime>("ExpirationDate");
				if let Ok(expire)=expire {
					s.insert("ExpirationDate", expire).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, e.into()))?;
					s.set_expiry(expire.and_utc());
					//println!("Expire at {}", expire);
				} else {
					s.insert("ExpirationDate", "").map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, e.into()))?;
					//println!("Expire Never");
				}
				let params = [(String::from("Active"), String::from("1"))].into();
				match db::users::get_users(&params) {
					Ok(users) => {
						let session_username=s.get::<String>("Username").unwrap();
						//println!("session_username: {}, users: {:?}", session_username, users);
						if !users.is_empty() {
							let u : Vec<&User> = users.iter().filter(|u|{u.Username == session_username}).collect();
							if u.len() == 1 {
								let _ = s.insert("Rights", u[0].Rights);
							}
						} else {
							// temporary rights
							let _ = s.insert("Rights", 2);
						}
					},
					Err(e) => {
						eprint!("Error while reading users to retreive session: {}", e);
					}
				}
				//println!("loaded session: {:?}", s);
				s.reset_data_changed();
				Ok(s)
			}
		);
		match session {
			Ok(s) => {
				self.inner
					.write()
					.await
					.insert(s.id().to_string(), s.clone());
				Ok(s.validate())
			},
			Err(e) => {
				eprintln!("Error while retreiving session: {}", e);
				Ok(None)
			}
		}
		//println!("{:?}",session);
		//Ok(session.ok().and_then(Session::validate))
	}

	async fn store_session(&self, session: Session) -> Result<Option<String>> {
		println!("storing session by id `{}`", session.id());
		self.inner
			.write()
			.await
			.insert(session.id().to_string(), session.clone());


		let connection = Connection::open("domorust.db").unwrap();
		let expiry = if let Some(date) = session.expiry() {
			date.naive_local().to_string()
		} else {
			"".to_string()
		};
		
		let res=connection.execute("REPLACE INTO UserSessions (SessionID, Username, AuthToken, ExpirationDate, RemoteHost, LastUpdate) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
			(
				session.id(),
				session.get::<String>("Username").unwrap_or_default(),
				session.get::<String>("AuthToken").unwrap_or_default(),
				expiry,
				session.get::<String>("RemoteHost").unwrap_or_default(),
				Utc::now().naive_local()
			)
		);
		if let Err(e)=res {
			eprintln!("Error while storing session: {}",e);
			return Err(e.into());
		}
		session.reset_data_changed();
		Ok(session.into_cookie_value())
	}

	async fn destroy_session(&self, session: Session) -> Result {
		println!("destroying session by id `{}`", session.id());
		let connection = Connection::open("domorust.db")?;
		let query="DELETE FROM UserSessions WHERE SessionID=?1";
		let _res = connection.execute(query, [session.id()])?;
		Ok(())
	}

	async fn clear_store(&self) -> Result {
		let connection = Connection::open("domorust.db")?;
		connection.execute("DELETE FROM UserSessions", ())?;
		Ok(())
	}
}
