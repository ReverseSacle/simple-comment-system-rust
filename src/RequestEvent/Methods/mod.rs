mod Get;
mod Post;
mod Put;
use super::CommonUse::{ 
	TcpStream,RequestMethod 
};

pub fn MethodHandle(methodtype: RequestMethod, url: &str,body: &str,stream: TcpStream){
	match methodtype{
		RequestMethod::GET => Get::get(url,stream),
		RequestMethod::POST => Post::post(body,stream),
		RequestMethod::PUT => Put::put(url,stream),
		_ => ()
	}
}
