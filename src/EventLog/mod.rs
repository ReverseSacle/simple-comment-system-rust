use super::Config::{
	prelude,
	Write,
	File,
	OpenOptions,
	create_dir_all,
	metadata
};
use chrono::prelude::*;

pub fn LogFileWrite(msg: &str,debug_info: bool)
{
	let mut flag = false;

	if metadata("./log").is_ok(){ 
		flag = true; 
	} else{
		if let Ok(_) = create_dir_all("./log"){
			flag = true;
		}
	}

	if flag{
		let path = "./log/httpserver.log";
	
		if let Ok(mut file) = OpenOptions::new()
						.create(true)
						.write(true)
						.append(true)
						.open(path){
			let log_content = format!("{}:{}\n",Local::now(),msg);

			if debug_info{ print!("{}",log_content.clone());}
			file.write_all(log_content.as_bytes());	
		} else { println!("Open log file error"); }
	}
}	

