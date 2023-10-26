use super::super::{ 
	CommonUse::{ 
		TcpStream,
		RequestPathType,
		EventLog
	},
	Status::{ 
		Response200,Response200_NF,Response200_DB,
	}
};

fn PathType(url: &str) -> Option<RequestPathType>{
	let mut select = [0;2];
	let mut switch = false;	

	for c in url.chars(){
		match c{
			'/' => { if switch { break; } else { switch = true; } },
			'D' | 'd' | 'T' | 't' | 'B' | 'b' | 'S' | 's' | 'E' | 'e' => select[0] += 1,
			'P' | 'p' | 'I' | 'i' => select[1] += 1,
			'A' | 'a' => {
				select[0] += 1;
				select[1] += 1;
			},
			_ => ()
		}
	}

	if 8 == select[0]{
		EventLog::LogFileWrite("RequestPathType: DATABASE",true);
		return Some(RequestPathType::DATABASE);
	}
	if 3 == select[1]{
		EventLog::LogFileWrite("RequestPathType: API",true);
		return Some(RequestPathType::API);
	}
	EventLog::LogFileWrite("RequestPathType: FILE",true);
	None
}
	 
pub fn get(url: &str, mut stream: TcpStream){
	match PathType(&url){
		Some(RequestPathType::DATABASE) => Response200_DB(stream),
		Some(RequestPathType::API) => Response200_NF(stream),
		_ => Response200(url,stream)
	}
}
