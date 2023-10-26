mod CommonUse;
mod Methods;
mod Status;
use CommonUse::{ 
	prelude::*,
	BufReader,
	Error,
	TcpListener,
	RequestMethod,
	EventLog
};

pub fn ServerListener_Default(port: u16) -> Result<TcpListener,Error>{
	match TcpListener::bind(&format!("{}:{}","0.0.0.0",port)){
		Ok(succeed) => {
			EventLog::LogFileWrite("TcpListener::bind() succeed...",true);
			Ok(succeed)
		},
		Err(err) => { 
			EventLog::LogFileWrite(&format!("TcpListener::bind() Fail.Err:{}",err),true);
			Err(err)
		}
	}
}

pub fn ServerListener(ipaddr: &str,port: u16) -> Result<TcpListener,Error>{
	match TcpListener::bind(&format!("{}:{}",ipaddr,port)){
		Ok(succeed) => {
			EventLog::LogFileWrite("TcpListener::bind() succeed...",true);
			Ok(succeed)
		},
		Err(err) => { 
			EventLog::LogFileWrite(&format!("TcpListener::bind() Fail.Err:{}",err),true);
			Err(err)
		}
	}
}

fn MethodType(head: &str) -> Option<RequestMethod>{
    let mut select = [0;3];

    for c in head.chars(){
        match c{
            'O' | 'o' | 'S' | 's' => select[0] += 1,
            'G' | 'g' | 'E' | 'e' => select[1] += 1,
            'U' | 'u' => select[2] += 1,
            'T' | 't' => {
                select[0] += 1;
                select[1] += 1;
            },
            'P' | 'p' => {
                select[0] += 1;
                select[2] += 1;
            },
            _ => break
        }
    }
    if 4 == select[0]{
		EventLog::LogFileWrite("Method: POST",true);
        return Some(RequestMethod::POST);
    }
    if 3 == select[1]{
		EventLog::LogFileWrite("Method: GET",true);
        return Some(RequestMethod::GET);
    }
    if 3 == select[2]{
		EventLog::LogFileWrite("Method: PUT",true);
        return Some(RequestMethod::PUT);
    }
	EventLog::LogFileWrite("Method: None",true);
    None
}

pub fn EventAccept(listener: TcpListener){
	EventLog::LogFileWrite("Waiting Connect...",true);
	for stream in listener.incoming(){
		if let Ok(mut succeed) = stream{
			EventLog::LogFileWrite("\nOne connection accepted",true);
			let mut reader = BufReader::new(&mut succeed);
			let mut buffer = String::new();

			if let Ok(_) = reader.read_line(&mut buffer){
				let mut head_method = String::new();
				let mut url = String::new();
				let mut flag = false;

				for c in buffer.chars(){
					if !flag && ' ' == c { flag = true; }
					else if !flag { head_method.push(c); }
					else if flag && ' ' == c { break; }
					else if flag { url.push(c); }
				}

				if let Some(methodtype) = MethodType(&head_method){
					let mut recv_size = 0;
					while 2 != buffer.len(){
						buffer.clear();
        				reader.read_line(&mut buffer).unwrap();
						if buffer.starts_with("Content-Length"){
							let size_split = buffer.split(":");
							for each in size_split{
								if !each.starts_with("Content-Length"){
									recv_size = each.trim().parse::<usize>().unwrap();
								}
							}
						}
					}
					let mut buffer = vec![0;recv_size];
					reader.read_exact(&mut buffer).unwrap();
					
					let body = String::from_utf8(buffer).unwrap();		
					Methods::MethodHandle(methodtype,&url,&body,succeed);
				}
			}
		}
	}
}

