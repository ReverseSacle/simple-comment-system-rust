extern crate httpserver;
use httpserver::RequestEvent;
 
fn main() {
	if let Ok(listener) = RequestEvent::ServerListener_Default(80){
		RequestEvent::EventAccept(listener);
	} else { println!("Build Server Listener Error"); }
}
