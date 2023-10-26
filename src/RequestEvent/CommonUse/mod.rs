pub use super::super::{
	Config::{
		prelude,
		BufReader,
		Read,Write,Error,
		File,metadata
	},
	EventLog
};
pub use std::{
	net::{ 
		TcpListener,TcpStream, 
	}
};
pub enum RequestMethod{
	GET,
	POST,
	PUT
}

pub enum RequestFileType{
	HTML,
	CSS,
	JS,
	PNG
}

pub enum RequestPathType{
	DATABASE,
	API
}
