use super::super::{
	CommonUse::{ 
		TcpStream
	},
	Status::{ 
		Response200_NF 
	},
	super::DatabaseEvent::{ 
		Args,Record,
		TableInsertQuery
	}
};

pub fn post(mut body: &str,mut stream: TcpStream){
	//println!("body:{}",body.clone());
	let args = Args{
        host: "127.0.0.1".to_string(),
        user: "root".to_string(),
        password: "@Killl611".to_string(),
        db_name: "CommentSystem".to_string(),
        table_name: "CommentBlock".to_string(),
    };
	let mut record = Record{
		nickname: String::new(),
		email: String::new(),
		content: String::new(),
		createat: String::new(),
	};

	let mut case = 0;
	for strs in body.split(':'){
		match case{
			0 => record.nickname.push_str(strs),
			1 => record.email.push_str(strs),
			2 => record.content.push_str(strs),
			_ => record.createat.push_str(strs)
		}
		case += 1;
	}
	TableInsertQuery(args,record);
	Response200_NF(stream);
}
