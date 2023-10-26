use mysql::{ 
	prelude::*,
	OptsBuilder,
	Pool,PooledConn,
	from_row
};
use super::EventLog;

pub struct Args{
	pub host: String,
	pub user: String,
	pub password: String,
	pub db_name: String,
	pub table_name: String
}

pub struct Record{
	pub nickname: String,
	pub email: String,
	pub content: String,
	pub createat: String
} 

pub fn TableInsertQuery(args: Args,record: Record){
	let opts = OptsBuilder::new()
				.ip_or_hostname(Some(args.host))
				.user(Some(args.user))
				.pass(Some(args.password))
				.db_name(Some(args.db_name));
	if let Ok(pool) = Pool::new(opts){
		if let Ok(mut connect) = pool.get_conn(){
			let query = format!("INSERT INTO {}(NickName,Email,Content,CreateAT)\nVALUES('{}','{}','{}','{}');",args.table_name,record.nickname,record.email,record.content,record.createat);

			EventLog::LogFileWrite(&format!("\nquery:\n{}",query.clone()),false);
			connect.query_drop(query).unwrap();
		}
	}
}

pub fn TableGetQuery(args: Args) -> Option<String>{
    let opts = OptsBuilder::new()
                .ip_or_hostname(Some(args.host))
                .user(Some(args.user))
                .pass(Some(args.password))
                .db_name(Some(args.db_name));

    if let Ok(pool) = Pool::new(opts){
        if let Ok(mut connect) = pool.get_conn(){
            let query = &format!("SELECT * FROM {} ORDER BY CreateAT ASC",args.table_name);
			let mut body = String::new();

			connect.query_iter(query).unwrap()
				.for_each(|row|{
					let r: (String,String,String,String) = from_row(row.unwrap());
					body.push_str(&format!("{}:{}:{}:{},",r.0,r.1,r.2,r.3));
			});	
			EventLog::LogFileWrite(&format!("\nQuery Body:\n{}",body.clone()),false);
			return Some(body);
        }
    }
	None
}
