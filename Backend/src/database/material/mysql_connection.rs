extern crate dotenv;
use dotenv::dotenv;
use std::env;

pub struct MySQLConnection {
  pub con: mysql::Pool
}

impl MySQLConnection {
  pub fn new(db_name: &str) -> Self
  {
    dotenv().ok();
    let mut dns: String = env::var("MYSQL_DNS").unwrap();
    dns.push_str(db_name);
    MySQLConnection {
      con: mysql::Pool::new(dns).unwrap()
    }
  }
}