use super::{
	CommonUse::{ 
		TcpStream,
		RequestFileType,
		Read,Write,
		File,metadata,
		EventLog
	},
	super::DatabaseEvent::{ 
		Args,Record,
		TableGetQuery
	}
};

fn FileType(url: &str) -> Option<RequestFileType>{
    let mut select = [0;4];

    for c in url.chars(){
        match c{
            'H' | 'h' | 'T' | 't' | 'M' | 'm' | 'L' | 'l' => select[0] += 1,
            'C' | 'c' => select[1] += 1,
            'J' | 'j' => select[2] += 1,
            'P' | 'p' | 'N' | 'n' | 'G' | 'g' => select[3] += 1,
            'S' | 's' => {
                select[1] += 1;
                select[2] += 1;
            },
            _ => break
        }
    }
    if 4 == select[0]{
		EventLog::LogFileWrite("RequestFileType: HTML",true);
        return Some(RequestFileType::HTML);
    }
    if 3 == select[1]{
		EventLog::LogFileWrite("RequestFileType: CSS",true);
        return Some(RequestFileType::CSS);
    }
    if 2 == select[2]{
		EventLog::LogFileWrite("RequestFileType: JS",true);
        return Some(RequestFileType::JS);
    }
    if 3 == select[3]{
		EventLog::LogFileWrite("RequestFileType: PNG",true);
        return Some(RequestFileType::PNG);
    }
	EventLog::LogFileWrite("RequestFileType: None",true);
    None
}

pub fn Response400(mut stream: TcpStream){
	stream.write_all("HTTP/1.0 400 BAD REQUEST\r\n\r\n".as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn Response404(mut stream: TcpStream){
	stream.write_all("HTTP/1.0 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn Response200(url: &str,mut stream: TcpStream){
	let mut real_path = if "/" == url { 
							"./docs/index.html".to_string()
						} else { format!("./docs{}",url) };

	EventLog::LogFileWrite(&format!("Real Path: {}",real_path.clone()),true);
	match metadata(&real_path){
		Ok(data) => {
			let filetype = url.chars().rev()
							.take_while(|&c| '.' != c && '/' != c)
							.collect::<String>();
			let filetype = FileType(&filetype);
    		let mut h_content_type = "Content-type: ".to_string();
    		let mut h_content_length = "Content-Length: ".to_string();

			match filetype{
				Some(RequestFileType::HTML) => h_content_type.push_str("text/html\r\n"),
				Some(RequestFileType::CSS) => h_content_type.push_str("text/css\r\n"),
				Some(RequestFileType::JS) => h_content_type.push_str("text/javascript\r\n"),
				Some(RequestFileType::PNG) => h_content_type.push_str("image/png\r\n"),
				_ => h_content_type.push_str("text/html\r\n")
			}
        	h_content_length.push_str(&format!("{}\r\n\r\n",data.len()));
		
			if let Ok(mut file) = File::open(real_path){
				let mut buffer = Vec::new();
				file.read_to_end(&mut buffer).unwrap();

				let h_status = "HTTP/1.0 200 OK\r\n".to_string();
    			let h_server = "Server: Linux\r\n".to_string();
    			let h_connect = "Connection: Close\r\n".to_string();
    			let response = format!("{}{}{}{}{}",
						h_status,h_server,h_connect,h_content_type,h_content_length);

    			stream.write_all(response.as_bytes()).unwrap();
				if let Some(RequestFileType::PNG) = filetype{
					stream.write_all(&buffer).unwrap();
				} else {
					stream.write_all(String::from_utf8_lossy(&buffer).as_bytes()).unwrap();
				}
    			stream.flush().unwrap();
			}
		},
		Err(err) => { 
			EventLog::LogFileWrite(&format!("Open Real Path Error: {}",err),true);
			Response404(stream); 
		}
	}
}

pub fn Response200_NF(mut stream: TcpStream){
	let h_status = "HTTP/1.0 200 OK\r\n".to_string();
    let h_server = "Server: Linux\r\n".to_string();
    let h_connect = "Connection: Close\r\n\r\n".to_string();

	let response = format!("{}{}{}",h_status,h_server,h_connect);
	stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn Response200_DB(mut stream: TcpStream){
	EventLog::LogFileWrite("Database Request...",true);
	
	let args = Args{
		host: "127.0.0.1".to_string(),
		user: "root".to_string(),
		password: "@Killl611".to_string(),
		db_name: "CommentSystem".to_string(),
		table_name: "CommentBlock".to_string(), 	
	};

	if let Some(body) = TableGetQuery(args){ 
		let h_status = "HTTP/1.0 200 OK\r\n".to_string();
    	let h_server = "Server: Linux\r\n".to_string();
    	let h_connect = "Connection: Close\r\n".to_string();
		let h_content_type = "Content-Type: text/plain\r\n".to_string();
		let h_content_length = format!("Content-Length: {}\r\n\r\n",body.len());

		let response = format!("{}{}{}{}{}",
					h_status,h_server,h_connect,h_content_type,h_content_length);

		stream.write_all(response.as_bytes()).unwrap();
		stream.write_all(body.as_bytes()).unwrap();
    	stream.flush().unwrap();
		EventLog::LogFileWrite("Database Request close",true);
	} else { 
		EventLog::LogFileWrite("Database Request Fail",true);
 	}
}	
