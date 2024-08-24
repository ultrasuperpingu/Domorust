
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use warp::{filters::ws::Message, Filter};

#[derive(Serialize, Deserialize)]
struct WSResp {
	#[serde(default)]
	pub data: String,
	pub event:String,//:"response"
	pub request:String,//:"request"
	pub requestid:i32//:1
}

impl Default for WSResp {
	fn default() -> Self {
		Self { data: Default::default(), event: "response".to_string(), request: "request".to_string(), requestid: -1 }
	}
}
pub(crate) fn websocket() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {

	warp::path("json")
		// The `ws()` filter will prepare the Websocket handshake.
		.and(warp::ws())
		.map(|ws: warp::ws::Ws| {
			// And then our closure will be called when it completes...
			ws.on_upgrade(|websocket| {
				println!("on_upgrade");
				handle_connection(websocket)
			})
		})
}

async fn handle_connection(websocket: warp::filters::ws::WebSocket) {
	let (mut tx, mut rx) = websocket.split();
	//let r=tx.send(Message::text("{\"coucou\":\"true\"  }")).await;
	//println!("result send: {:?}", r);
	tokio::task::spawn(async move {
		while let Some(result) = rx.next().await {
			let msg = match result {
				Ok(msg) => msg,
				Err(e) => {
					eprintln!("websocket error: {}", e);
					break;
				}
			};
			let msg = if let Ok(s) = msg.to_str() {
				s
			} else {
				return;
			};
			println!("received {:?}", msg);
			let request: Result<WSResp, serde_json::Error>=serde_json::from_str(msg);
			let mut resp=WSResp::default();
			if let Ok(request) = request {
				//TODO: answer
				resp.requestid = request.requestid;
			} else {
				resp.data="Error reading message".to_string();
			}
			let test =serde_json::to_string(&resp).unwrap();
			tx.send(Message::text(test))
				.await.unwrap_or_else(|e| {
				eprintln!("websocket send error: {}", e);
				}
			);
		}
	});
}
//{"event":"request","requestid":1,"query":"type=command&param=getdevices&filter=all&used=true&favorite=1&order=[Order]&plan=0&lastupdate=1722877239"}